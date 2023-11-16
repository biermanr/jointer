import nox


@nox.session()
def test(session: nox.Session) -> None:
    """Run tests with pytest."""
    session.install("maturin", "pytest")
    session.run("maturin", "develop")
    session.run("pytest")


@nox.session()
def rs_unit_test(session: nox.Session) -> None:
    """Run rust unit tests with cargo."""
    session.install("cargo")
    session.run("cargo", "test")


@nox.session()
def black(session):
    session.run("black", "python", external=True)


@nox.session()
def lint(session):
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
