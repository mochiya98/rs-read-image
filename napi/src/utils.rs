extern crate napi;
extern crate napi_derive;

use napi::*;

#[inline(always)]
pub fn uint8array_from_unknown(env: &napi::Env, src: &JsUnknown) -> Result<JsTypedArray> {
  let js_uint8array = unsafe {
    env
      .get_global()?
      .get_named_property::<JsFunction>("Uint8Array")?
      .into_unknown()
      .cast::<JsObject>()
  };
  let js_uint8array_from = js_uint8array.get_named_property::<JsFunction>("from")?;
  let result = unsafe {
    js_uint8array_from
      .call(Some(&js_uint8array), &[src])?
      .cast::<JsTypedArray>()
  };
  Ok(result)
}
#[inline(always)]
pub fn uint8array_from_vec(env: &napi::Env, src: Vec<u8>) -> Result<JsTypedArray> {
  let len = Vec::len(&src);
  Ok(
    env
      .create_arraybuffer_with_data(src)?
      .into_raw()
      .into_typedarray(TypedArrayType::Uint8, len, 0)?,
  )
}
