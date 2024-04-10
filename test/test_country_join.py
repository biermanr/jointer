from jointer import jointer


def test_code_pop(tmp_path, capsys):
    codes_path = tmp_path / "country_codes.tsv"
    codes_path.write_text(
        "country_code	country_name\n"
        + "BRA	Brazil\n"
        + "CHN	China\n"
        + "IND	India\n"
        + "USA	United-States\n"
    )

    pops_path = tmp_path / "country_populations.tsv"
    pops_path.write_text(
        "country_code	population\n"
        + "BRA	209\n"
        + "CHN	1439\n"
        + "IND	1380\n"
        + "USA	328\n"
    )

    joint_text = (
        "country_code	country_name	population\n"
        + "BRA	Brazil	209\n"
        + "CHN	China	1439\n"
        + "IND	India	1380\n"
        + "USA	United-States	328\n"
    )

    jointer.join(codes_path, pops_path)
    captured = capsys.readouterr()

    assert captured.out == joint_text
