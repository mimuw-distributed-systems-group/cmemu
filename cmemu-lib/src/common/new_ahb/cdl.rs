#![cfg(feature = "cycle-debug-logger")]

pub const DEFAULT_STR: &str = "?unknown";

#[allow(unused)]
mod just_tag {
    #[cfg(test)]
    use crate::common::new_ahb::test::utils::wildcard_eq;
    #[cfg(test)]
    use owo_colors::OwoColorize;
    use std::fmt::{Debug, Display, Formatter};

    #[cfg_attr(test, derive(Eq))]
    #[derive(Clone)]
    pub(crate) struct CdlTag(&'static str);

    impl Default for CdlTag {
        fn default() -> Self {
            CdlTag(super::DEFAULT_STR)
        }
    }

    impl Display for CdlTag {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            #[cfg(not(test))]
            {
                f.write_str(self.0)
            }
            #[cfg(test)]
            {
                write!(f, "{}", self.0.yellow())
            }
        }
    }

    impl Debug for CdlTag {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt(self, f)
        }
    }

    impl CdlTag {
        pub(crate) fn reply_trace(&self, _responder: &'static str) -> Self {
            Self(self.0)
        }

        pub const fn from_const(s: &'static str) -> Self {
            CdlTag(s)
        }

        pub const fn as_str(&self) -> &'static str {
            self.0
        }
    }

    impl From<&'static str> for CdlTag {
        fn from(s: &'static str) -> Self {
            CdlTag(s)
        }
    }

    #[cfg(any(test, debug_assertions))]
    impl<'a> PartialEq<&'a str> for CdlTag {
        fn eq(&self, other: &&'a str) -> bool {
            #[cfg(test)]
            {
                wildcard_eq(self.0, other)
            }
            #[cfg(not(test))]
            {
                self.0 == *other
            }
        }
    }

    #[cfg(any(test, debug_assertions))]
    impl PartialEq<CdlTag> for CdlTag {
        fn eq(&self, other: &CdlTag) -> bool {
            self == &other.0
        }
    }

    #[cfg(test)]
    mod test {
        use super::CdlTag;

        #[test]
        fn cdl_tag_matching() {
            assert_eq!(CdlTag::default(), CdlTag::from_const("*"));
        }
    }
}

#[cfg(feature = "cdl-ahb-trace")]
mod trace {
    use super::just_tag::CdlTag as JustTag;
    use crate::common::new_ahb::cdl::DEFAULT_STR;
    use crate::common::new_ahb::ports::AHBPortConfig;
    use crate::confeature::cdl_ahb_trace as conf;
    use crate::engine::{Context, Timepoint};
    use crate::utils::IfExpr;
    use crate::utils::{dife, ife};
    use itertools::Itertools;
    use owo_colors::AnsiColors::{Green, Red};
    use owo_colors::OwoColorize;
    use std::borrow::Cow;
    use std::fmt::{Debug, Display, Formatter};
    use std::ops::Deref;
    use std::sync::atomic::{AtomicU32, Ordering};

    static UNIQ_ID: AtomicU32 = AtomicU32::new(1);

    #[cfg_attr(test, derive(PartialEq, Eq))]
    #[derive(Clone, Debug)]
    enum History {
        Simple { n_clones: u8, n_hops: u8 },
        Full(Vec<Trace>),
    }
    impl History {
        fn as_full(&self) -> &Vec<Trace> {
            match self {
                History::Full(v) => v,
                History::Simple { .. } => panic!("Not full trace"),
            }
        }
        fn as_full_mut(&mut self) -> &mut Vec<Trace> {
            match self {
                History::Full(v) => v,
                History::Simple { .. } => panic!("Not full trace"),
            }
        }
    }

    #[cfg_attr(test, derive(Eq))]
    pub(crate) struct CdlTag {
        id: u32,
        origin: JustTag,
        history: History,
        delivered: Option<JustTag>,
    }

    impl Display for CdlTag {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}#{}", self.as_str(), self.id)
        }
    }

    impl Debug for CdlTag {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            // #[cfg(feature = "pretty_log")]
            write!(f, "[")?;
            write!(f, "#{}", self.id.bright_green())?;
            if *conf::TRACE_CLONES {
                write!(
                    f,
                    "{}",
                    dife(
                        self.count_clones() <= 3,
                        (0..self.count_clones()).map(|_| "'").format(""),
                        &format_args!("'{}", self.count_clones())
                    )
                )?;
            }
            write!(f, " ")?;
            if *conf::DISPLAY_STATUS {
                write!(
                    f,
                    "{} ",
                    self.delivered.ife("D".color(Green), "T".color(Red)),
                )?;
            }
            write!(f, "{}", self.origin.yellow(),)?;

            if *conf::DISPLAY_TRACE {
                write!(
                    f,
                    "{}{}",
                    ife(self.history.as_full().is_empty(), "", " => "),
                    self.history.as_full().iter().format(" -> "),
                )?;
            } /*else if let Some(Trace::Stamp(_, dst, _)) = self.history.last() {
            write!(f, " =..> {}", dst.deref().bright_yellow())?;
            }*/

            if *conf::DISPLAY_STATUS {
                write!(
                    f,
                    "{}{}",
                    dife(
                        self.delivered.is_some(),
                        dife(
                            *conf::DISPLAY_TRACE,
                            " => ",
                            &format_args!(" ={}> ", self.count_hops())
                        ),
                        ""
                    ),
                    self.delivered.iter().format("").cyan()
                )?;
            }
            write!(f, "]")
        }
    }

    impl Display for Trace {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Trace::Stamp(src, dst, t) => {
                    if *conf::DISPLAY_DEST {
                        write!(f, "[")?;
                    }
                    write!(f, "{}", src.deref().bright_yellow())?;
                    if *conf::DISPLAY_TIME {
                        write!(f, "@{}", t.as_picos().bright_green())?;
                    }
                    if *conf::DISPLAY_DEST {
                        write!(
                            f,
                            "{}{}]",
                            "->".bright_white().bold(),
                            dst.deref().bright_yellow()
                        )?;
                    }
                }
                Trace::Cloned => write!(f, "{}", "Cloned".magenta())?,
            }
            Ok(())
        }
    }

    impl Clone for CdlTag {
        fn clone(&self) -> Self {
            let mut history = self.history.clone();
            if *conf::TRACE_CLONES {
                match &mut history {
                    History::Full(history) => history.push(Trace::Cloned),
                    History::Simple { n_clones, .. } => *n_clones += 1,
                }
            }
            Self {
                id: self.id,
                origin: self.origin.clone(),
                history,
                delivered: self.delivered.clone(),
            }
        }
    }
    #[derive(Debug, PartialEq, Eq, Clone)]
    // struct Stamp(JustTag, JustTag, Timepoint);
    enum Trace {
        Stamp(Cow<'static, str>, Cow<'static, str>, Timepoint),
        Cloned,
    }

    impl CdlTag {
        pub(crate) fn count_hops(&self) -> usize {
            match &self.history {
                History::Full(vec) => vec
                    .iter()
                    .filter(|tr| matches!(tr, Trace::Stamp(..)))
                    .count(),
                History::Simple { n_hops, .. } => *n_hops as usize,
            }
        }
        pub(crate) fn count_clones(&self) -> usize {
            match &self.history {
                History::Full(vec) => vec.iter().filter(|tr| matches!(tr, Trace::Cloned)).count(),
                History::Simple { n_clones, .. } => *n_clones as usize,
            }
        }
        pub(crate) fn get_id(&self) -> u32 {
            self.id
        }

        pub(crate) fn reply_trace(&self, responder: &'static str) -> Self {
            let mut history = self.history.clone();
            if let History::Full(ref mut history) = history {
                history.shrink_to_fit();
            }
            Self {
                id: self.id,
                origin: self.origin.clone(),
                history,
                delivered: Some(responder.into()),
            }
        }
        pub const fn from_const(s: &'static str) -> Self {
            CdlTag {
                id: 0,
                origin: JustTag::from_const(s),
                history: if *conf::DISPLAY_TRACE || *conf::ALWAYS_TRACE_STAMPS {
                    History::Full(vec![])
                } else {
                    History::Simple {
                        n_clones: 0,
                        n_hops: 0,
                    }
                },
                delivered: None,
            }
        }
        pub const fn as_str(&self) -> &'static str {
            self.origin.as_str()
        }

        pub(crate) fn stamp<SRC, DST>(&mut self, ctx: &Context)
        where
            SRC: AHBPortConfig,
            DST: AHBPortConfig,
        {
            if !(*conf::DISPLAY_TRACE || *conf::ALWAYS_TRACE_STAMPS) {
                if let History::Simple { ref mut n_hops, .. } = self.history {
                    *n_hops += 1;
                }
                // make it clear the following is dead code if !TRACE_STAMPS
                return;
            }
            self.history.as_full_mut().push(Trace::Stamp(
                if *conf::USE_TYPE_NAME {
                    p(std::any::type_name::<SRC>()).into()
                } else {
                    SRC::TAG.into()
                },
                if *conf::USE_TYPE_NAME {
                    p(std::any::type_name::<DST>()).into()
                } else {
                    DST::TAG.into()
                },
                ctx.event_queue().get_current_time(),
            ));
        }
    }

    // string from type
    pub fn p(s: &'static str) -> String {
        // TODO: make some nice filtering
        s.replace("cmemu_lib::common::new_ahb::", "")
    }

    impl From<&'static str> for CdlTag {
        fn from(s: &'static str) -> Self {
            CdlTag {
                id: self::UNIQ_ID.fetch_add(1, Ordering::Relaxed),
                ..CdlTag::from_const(s)
            }
        }
    }

    impl From<JustTag> for CdlTag {
        fn from(s: JustTag) -> Self {
            CdlTag {
                origin: s,
                ..Default::default()
            }
        }
    }

    impl Default for CdlTag {
        fn default() -> Self {
            CdlTag::from(DEFAULT_STR)
        }
    }

    #[cfg(any(test, debug_assertions))]
    impl<'a> PartialEq<&'a str> for CdlTag {
        fn eq(&self, other: &&'a str) -> bool {
            self.origin == *other
        }
    }

    #[cfg(any(test, debug_assertions))]
    impl PartialEq<CdlTag> for CdlTag {
        fn eq(&self, other: &CdlTag) -> bool {
            self.origin == other.origin
        }
    }

    #[cfg(any(test, debug_assertions))]
    impl PartialEq<JustTag> for CdlTag {
        fn eq(&self, other: &JustTag) -> bool {
            &self.origin == other
        }
    }

    #[cfg(test)]
    mod test {
        use super::CdlTag;

        #[test]
        fn cdl_tag_matching() {
            assert_eq!(CdlTag::default(), CdlTag::from("*"));
        }
    }
}

#[cfg(not(feature = "cdl-ahb-trace"))]
pub(crate) use just_tag::CdlTag;
#[cfg(feature = "cdl-ahb-trace")]
pub(crate) use trace::CdlTag;

impl CdlTag {
    pub const DEFAULT_STR: &'static str = DEFAULT_STR;

    #[cfg(any(test, debug_assertions))]
    #[allow(dead_code)]
    pub fn eq_if_present(&self, other: &Self) -> bool {
        self == other || self == &Self::DEFAULT_STR
    }
}
