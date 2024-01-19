use snafu::{ResultExt, Snafu};
use std::error::Error;
use thiserror::Error;
use tracing::{debug, error, info, span, trace, warn, Level};


// the `#[tracing::instrument]` attribute creates and enters a span
// every time the instrumented function is called. The span is named after
// the function or method. Parameters passed to the function are recorded as fields.
#[tracing::instrument]
pub fn write(yak: usize) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // this creates an event at the TRACE log level with two fields:
    // - `excitement`, with the key "excitement" and the value "yay!"
    // - `message`, with the key "message" and the value "hello! I'm gonna shave a yak."
    //
    // unlike other fields, `message`'s shorthand initialization is just the string itself.
    trace!(excitement = "log!");
    if yak == 3 {
        warn!("could not locate log!");
        return OutOfCash
            .fail()
            .map_err(|source| MissingYakError::OutOfSpace { source })
            .context(MissingYak)
            .map_err(|err| err.into());
    } else {
        trace!("log! saved successfully");
    }
    Ok(())
}

pub fn write_all(yaks: usize) -> usize {
    // Constructs a new span named "shaving_yaks" at the INFO level,
    // and a field whose key is "yaks". This is equivalent to writing:
    //
    // let span = span!(Level::INFO, "shaving_yaks", yaks = yaks);
    //
    // local variables (`yaks`) can be used as field values
    // without an assignment, similar to struct initializers.
    let span = span!(Level::INFO, "logging_msg", yaks);
    let _enter = span.enter();

    info!("Writing log");

    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        let res = write(yak);
        debug!(target: "log_events", yak, shaved = res.is_ok());

        if let Err(ref error) = res {
            // Like spans, events can also use the field initialization shorthand.
            // In this instance, `yak` is the field being initialized.
            error!(yak, error = error.as_ref(), "failed to log message");
        } else {
            yaks_shaved += 1;
        }
        trace!(yaks_shaved);
    }

    yaks_shaved
}

// Error types
// Usually you would pick one error handling library to use, but they can be mixed freely
#[derive(Debug, Snafu)]
enum OutOfSpaceError {
    #[snafu(display("out of space"))]
    OutOfCash,
}

#[derive(Debug, Error)]
enum MissingYakError {
    #[error("missing log")]
    OutOfSpace { source: OutOfSpaceError },
}

#[derive(Debug, Snafu)]
enum YakError {
    #[snafu(display("missing log"))]
    MissingYak { source: MissingYakError },
}