extern crate napi;
extern crate napi_derive;

mod utils;

use napi::*;
use napi_derive::*;

#[js_function(1)]
fn read_image(ctx: CallContext) -> Result<JsTypedArray> {
  if ctx.length < 1 {
    return Err(Error::from_reason("invalid_argument".to_string()));
  }
  let raw_src = ctx.get::<JsUnknown>(0)?;
  let typed_src = if raw_src.is_typedarray()? {
    unsafe { raw_src.cast::<JsTypedArray>() }
  } else {
    utils::uint8array_from_unknown(&*(ctx.env), &raw_src)?
  };
  let vec_src = AsRef::<[u8]>::as_ref(&typed_src.into_value()?).to_vec();
  match read_image_lib::read_image(vec_src) {
    Ok(buf) => Ok(utils::uint8array_from_vec(ctx.env, buf)?),
    Err(e) => Err(Error::from_reason(e.to_string())),
  }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("readImage", read_image)?;

  Ok(())
}
