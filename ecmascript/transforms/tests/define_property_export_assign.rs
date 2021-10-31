#![cfg(all(
    feature = "swc_ecma_transforms_module",
    feature = "swc_ecma_transforms_typescript",
))]

use swc_common::{chain, Mark};
use swc_ecma_parser::Syntax;
use swc_ecma_transforms::typescript::strip;
use swc_ecma_transforms_module::common_js::common_js;
use swc_ecma_transforms_testing::test;

fn syntax() -> Syntax {
    Syntax::Typescript(Default::default())
}

test!(
    syntax(),
    |_| chain!(
        common_js(Mark::fresh(Mark::root()), Default::default(), None),
        strip()
    ),
    function_export_define_property_assign,
    r#"
export function warn() {
  throw new Error("this should not be called");
}

export const test = {};
export const test2 = {};

Object.defineProperty(test, "warn", {
  get: () => warn,
  set: (v) => {
    (warn as any) = v;
  },
});
Object.defineProperty(test2, "work", {
  get: () => warn,
  set: (v) => {
    warn = v;
  },
});

"#,
    r#"
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.warn = warn;
exports.test2 = exports.test = void 0;
function warn() {
    throw new Error("this should not be called");
}
const test = {
};
exports.test = test;
const test2 = {
};
exports.test2 = test2;
Object.defineProperty(test, "warn", {
    get: ()=>warn
    ,
    set: (v)=>{
        exports.warn = warn = v;
    }
});
Object.defineProperty(test2, "work", {
  get: ()=>warn
  ,
  set: (v)=>{
      exports.warn = warn = v;
  }
});

"#
);
