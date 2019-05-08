extern crate snafu;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    Leaf {
        name: String,
    },

    BoxedSelf {
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },

    BoxedPublic {
        #[snafu(source(from(ApiError, Box::new)))]
        source: Box<ApiError>,
    },
}

#[derive(Debug, Snafu)]
#[snafu(source(from(Error, Box::new)))]
struct ApiError(Box<Error>);

type Result<T, E = Error> = std::result::Result<T, E>;

fn lookup() -> Result<()> {
    Leaf { name: "foo" }.fail()
}

fn add() -> Result<()> {
    lookup().eager_context(BoxedSelf)
}

fn public() -> Result<(), ApiError> {
    add()?;
    Ok(())
}

fn re_private() -> Result<()> {
    public().eager_context(BoxedPublic)
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
    re_private().unwrap_err();
}
