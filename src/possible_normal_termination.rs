/// This is a type that is intended to allow a function to return `Terminate`
/// when the program should terminate in a non-erroring way or `Continue` when
/// the program should not terminate as a result of the function call. This type
/// exists because destructors are not called when exiting through
/// std::process::exit.
/// 
/// I am choosing to make this a new type rather than alias Result<_, ()> for
/// type safety (there are significant differences between when you should
/// ordinarily use Result and when you should use this).
#[must_use]
pub enum PossibleNormalTermination<NonTerminationValue> {
    Continue(NonTerminationValue),
    Terminate,
}