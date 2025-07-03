/// Basic event types used by the parser.
#[derive(Clone, Debug)]
pub struct FrameDone;

#[derive(Clone, Debug)]
pub struct MatchStart;

#[derive(Clone, Debug)]
pub struct RoundStart;

#[derive(Clone, Debug)]
pub struct RoundEnd;

#[derive(Clone, Debug)]
pub struct DataTablesParsed;
