//! The versions of a conversation's turns.
//!
//! Trying a reply again, or editing a prompt, forks the session before that
//! turn: the fork shares everything before it and writes the turn anew. The
//! session and its forks at one point are the versions of that turn, and
//! each fork may have versions of its own later turns. A conversation is
//! therefore a tree of sessions; what shows is one path through it.

use crate::schema::{Seq, SessionId};

/// One session of a conversation, as the host lists it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Branch {
    /// The session.
    pub id: SessionId,
    /// The session it was forked from and the last `seq` it shares.
    pub forked_from: Option<(SessionId, Seq)>,
    /// When it was created, in milliseconds.
    pub created_at: u64,
    /// When it last changed, in milliseconds.
    pub updated_at: u64,
}

/// The versions of one turn: `parent` wrote it first, after `seq`, and each
/// other session is a fork of `parent` at `seq` that wrote it again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    /// The session the turn was first written in.
    pub parent: SessionId,
    /// The last `seq` of `parent` every version shares.
    pub seq: Seq,
    /// The versions, oldest first: `parent`, then its forks at `seq`.
    pub sessions: Vec<SessionId>,
    /// Which one shows.
    pub current: usize,
}

impl Slot {
    /// The session that shows.
    pub fn shown(&self) -> &SessionId {
        &self.sessions[self.current]
    }
}

/// What a conversation shows: each session along the path with the `seq` to
/// fold it to (`None` for the last, which is followed live), and the
/// versions met on the way, in order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Path {
    /// The sessions shown, from the root, each with the `seq` it ends at.
    pub sessions: Vec<(SessionId, Option<Seq>)>,
    /// The turns with more than one version, in the order they show.
    pub slots: Vec<Slot>,
}

/// The path from `root` to the version most recently worked on.
///
/// `branches` is every session the host listed; forks of sessions outside
/// the conversation are ignored. Along the path, every turn that has forks
/// gets a slot, showing the version on the path, or the first one where
/// the path does not pass.
pub fn plan(root: &SessionId, branches: &[Branch]) -> Path {
    let forks_of = |parent: &SessionId| -> Vec<&Branch> {
        let mut forks: Vec<&Branch> = branches
            .iter()
            .filter(|b| b.forked_from.as_ref().is_some_and(|(p, _)| p == parent))
            .collect();
        forks.sort_by(|a, b| a.created_at.cmp(&b.created_at).then(a.id.cmp(&b.id)));
        forks
    };
    // The newest session reachable from the root.
    let mut family = vec![root.clone()];
    let mut index = 0;
    while index < family.len() {
        let next: Vec<SessionId> = forks_of(&family[index])
            .iter()
            .map(|b| b.id.clone())
            .collect();
        family.extend(next);
        index += 1;
    }
    // Last changed, then last made: a fork is newer than what it forked.
    let age = |id: &SessionId| {
        branches
            .iter()
            .find(|b| &b.id == id)
            .map_or((0, 0), |b| (b.updated_at, b.created_at))
    };
    let leaf = family
        .iter()
        .enumerate()
        .max_by(|(i, a), (j, b)| age(a).cmp(&age(b)).then(i.cmp(j)))
        .map(|(_, id)| id)
        .cloned()
        .unwrap_or_else(|| root.clone());
    // Walk back from the leaf to the root.
    let mut chain = vec![leaf];
    while chain.last() != Some(root) {
        let last = chain.last().cloned().unwrap_or_else(|| root.clone());
        let parent = branches
            .iter()
            .find(|b| b.id == last)
            .and_then(|b| b.forked_from.as_ref())
            .map(|(p, _)| p.clone());
        match parent {
            Some(parent) => chain.push(parent),
            None => break,
        }
    }
    chain.reverse();

    let mut path = Path::default();
    for (at, session) in chain.iter().enumerate() {
        let next = chain.get(at + 1);
        let leave = next.and_then(|child| {
            branches
                .iter()
                .find(|b| &b.id == child)
                .and_then(|b| b.forked_from.as_ref())
                .map(|(_, seq)| *seq)
        });
        path.sessions.push((session.clone(), leave));
        // Every fork point of this session within the part that shows.
        let mut points: Vec<Seq> = forks_of(session)
            .iter()
            .filter_map(|b| b.forked_from.as_ref().map(|(_, seq)| *seq))
            .filter(|seq| leave.is_none_or(|leave| *seq <= leave))
            .collect();
        points.sort_unstable();
        points.dedup();
        for seq in points {
            let mut sessions = vec![session.clone()];
            sessions.extend(
                forks_of(session)
                    .iter()
                    .filter(|b| b.forked_from.as_ref().is_some_and(|(_, s)| *s == seq))
                    .map(|b| b.id.clone()),
            );
            let current = next
                .filter(|_| leave == Some(seq))
                .and_then(|child| sessions.iter().position(|s| s == child))
                .unwrap_or(0);
            path.slots.push(Slot {
                parent: session.clone(),
                seq,
                sessions,
                current,
            });
        }
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(name: &str) -> SessionId {
        SessionId::new(name)
    }

    fn branch(name: &str, from: Option<(&str, Seq)>, at: u64) -> Branch {
        Branch {
            id: id(name),
            forked_from: from.map(|(p, s)| (id(p), s)),
            created_at: at,
            updated_at: at,
        }
    }

    #[test]
    fn a_conversation_without_forks_is_its_root() {
        let path = plan(&id("a"), &[branch("a", None, 1)]);
        assert_eq!(path.sessions, [(id("a"), None)]);
        assert!(path.slots.is_empty());
    }

    #[test]
    fn the_newest_version_shows_and_its_siblings_are_listed() {
        let branches = [
            branch("a", None, 1),
            branch("b", Some(("a", 9)), 2),
            branch("c", Some(("a", 9)), 3),
            branch("other", Some(("z", 9)), 9),
        ];
        let path = plan(&id("a"), &branches);
        assert_eq!(path.sessions, [(id("a"), Some(9)), (id("c"), None)]);
        assert_eq!(path.slots.len(), 1);
        assert_eq!(path.slots[0].sessions, [id("a"), id("b"), id("c")]);
        assert_eq!(path.slots[0].shown(), &id("c"));
    }

    #[test]
    fn at_the_same_time_a_fork_is_newer_than_what_it_forked() {
        let branches = [branch("a", None, 5), branch("b", Some(("a", 1)), 5)];
        let path = plan(&id("a"), &branches);
        assert_eq!(path.slots[0].shown(), &id("b"));
    }

    #[test]
    fn a_version_of_a_later_turn_in_a_fork_nests_under_it() {
        let mut branches = vec![
            branch("a", None, 1),
            branch("b", Some(("a", 9)), 2),
            branch("c", Some(("b", 20)), 5),
        ];
        let path = plan(&id("a"), &branches);
        let shown: Vec<_> = path.sessions.iter().map(|(s, _)| s.clone()).collect();
        assert_eq!(shown, [id("a"), id("b"), id("c")]);
        assert_eq!(path.slots.len(), 2);
        assert_eq!(path.slots[1].parent, id("b"));
        assert_eq!(path.slots[1].current, 1);

        // Once the root is the newest, the path stays on it and every
        // version of its turns shows its first.
        branches[0].updated_at = 10;
        let path = plan(&id("a"), &branches);
        assert_eq!(path.sessions, [(id("a"), None)]);
        assert_eq!(path.slots[0].current, 0);
    }
}
