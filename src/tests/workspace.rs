mod local {
    use once_cell::sync::Lazy;
    use std::rc::Rc;

    use tempdir::TempDir;

    use crate::workspace::{
        fs::LocalWorkspace, AsComponents, Component, IntoComponents, Root, Workspace, WorkspaceRoot,
    };

    type LocalComponent = <LocalWorkspace as Workspace>::Component;

    #[test]
    fn loc() {
        assert_eq!(
            loc!(["a", &["b".into(), "c".into()] as &[LocalComponent]] as LocalComponent)
                .into_components()
                .as_components(),
            &["a".into(), "b".into(), "c".into()]
        );
    }

    #[test]
    fn loc_root() {
        root!(type TempRoot: WorkspaceRoot = ["temp"]);
        assert_eq!(
            loc_root!(TempRoot, ["a", "b"] as LocalComponent)
                .into_components()
                .as_components(),
            &["temp".into(), "a".into(), "b".into()]
        );
    }

    static LOCAL_WORKSPACE: Lazy<LocalWorkspace> =
        Lazy::new(|| LocalWorkspace::new(TempDir::new("remediate").unwrap().into_path()));

    #[test]
    #[should_panic]
    fn write_root() {
        LOCAL_WORKSPACE.write::<WorkspaceRoot, _>((), b"");
    }

    #[test]
    fn read() {
        LOCAL_WORKSPACE.write::<WorkspaceRoot, _>(loc!(["a", "b"] as LocalComponent), b"Hello");
        assert_eq!(
            LOCAL_WORKSPACE
                .read::<WorkspaceRoot, _>(loc!(["a", "b"] as LocalComponent))
                .as_ref(),
            b"Hello"
        );
    }
}
