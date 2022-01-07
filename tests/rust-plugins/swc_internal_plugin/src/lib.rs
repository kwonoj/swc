use rkyv::{AlignedVec, Deserialize};
use swc_plugin::{Program, VisitMut, Expr, ExprStmt, Stmt, ExprOrSuper, MemberExpr, Lit, Str, StrKind, CallExpr, DUMMY_SP, JsWord, as_folder, FoldWith};

struct Dummy;

impl VisitMut for Dummy {
    fn visit_mut_stmt(&mut self, n: &mut Stmt) {
        match n {
            Stmt::Expr(ExprStmt { expr, .. }) => match &mut **expr {
                Expr::Call(CallExpr { callee, args, .. }) => match callee {
                    ExprOrSuper::Expr(expr) => match &**expr {
                        Expr::Member(MemberExpr { obj, .. }) => match obj {
                            ExprOrSuper::Expr(expr) => {
                                if let Some(ident) = expr.clone().ident() {
                                    if ident.sym == *"console" {
                                        args[0].expr = Box::new(Expr::Lit(Lit::Str(Str {
                                            span: DUMMY_SP,
                                            has_escape: false,
                                            kind: StrKind::default(),
                                            value: JsWord::from("changed"),
                                        })));
                                    }
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        };
    }
}

#[no_mangle]
// TODO:
// - this is not complete signature, need to take config_json
// - swc_plugin macro to support ergonomic interfaces
// - better developer experience: println / dbg!() doesn't emit anything
// - typed json config instead of str, which requires additional deserialization
pub fn process(ast_ptr: *mut u8, len: u32) -> (i32, i32) {
    let mut vec = AlignedVec::with_capacity(len.try_into().unwrap());
    let v = unsafe { std::slice::from_raw_parts(ast_ptr, len.try_into().unwrap()) };
    vec.extend_from_slice(v);

    // TODO: trait bound complaining in deserialize_for_plugin<T>
    let archived = unsafe { rkyv::archived_root::<Program>(&vec[..]) };
    let v: Program = archived.deserialize(&mut rkyv::Infallible).unwrap();

    let mut folder = as_folder(Dummy);
    let v = v.fold_with(&mut folder);

    let serialized = rkyv::to_bytes::<_, 512>(&v).unwrap();
    //
    let bbb = serialized.as_ptr();
    let _ = core::mem::ManuallyDrop::new(bbb);
    (bbb as _, serialized.len().try_into().unwrap())
}
