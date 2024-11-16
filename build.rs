use gh_workflow::release_plz::Release;
use gh_workflow::toolchain::Toolchain;
use gh_workflow::*;

fn main() {
    let flags = RustFlags::deny("warnings");
    let event = Event::default()
        .push(Push::default().add_branch("main"))
        .pull_request_target(
            PullRequestTarget::default()
                .add_type(PullRequestType::Opened)
                .add_type(PullRequestType::Synchronize)
                .add_type(PullRequestType::Reopened)
                .add_branch("main"),
        );

    let build = Job::new("Build and Test")
        .add_step(Step::checkout())
        .add_step(
            Toolchain::default()
                .add_stable()
                .add_nightly()
                .add_clippy()
                .add_fmt(),
        )
        .add_step(
            Cargo::new("build")
                .args("--all-features --workspace")
                .name("Cargo Build"),
        )
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("--check")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .args("--all-features --workspace -- -D warnings")
                .name("Cargo Clippy"),
        );

    let permissions = Permissions::default()
        .pull_requests(Level::Write)
        .packages(Level::Write)
        .contents(Level::Write);

    let release = Job::new("Release")
        .needs("build")
        .add_env(Env::github())
        .add_env(Env::new(
            "CARGO_REGISTRY_TOKEN",
            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
        ))
        .permissions(permissions)
        .add_step(Step::checkout())
        .add_step(Release::default());

    Workflow::new("Build and Release")
        .add_env(flags)
        .on(event)
        .add_job("build", build)
        .add_job("release", release)
        .generate()
        .unwrap();
}
