import nox


@nox.session()
def black(session):
    session.run("black", "python", external=True)


@nox.session()
def py_lint(session):
    """Lint code with flake8."""
    session.install(
        "flake8",
        "flake8-annotations",
        "flake8-black",
        "flake8-bugbear",
        "flake8-docstrings",
        "flake8-import-order",
        "darglint",
    )
    session.run("flake8")


@nox.session()
def rs_lint(session):
    """Lint rust code with clippy."""
    session.run("cargo", "test", external=True)


@nox.session()
def py_test(session: nox.Session) -> None:
    """Run tests with pytest."""
    session.run("maturin", "develop", external=True)
    session.run("pytest", external=True)


@nox.session()
def rs_unit_test(session: nox.Session) -> None:
    """Run rust unit tests with cargo."""
    session.run("cargo", "test", external=True)
