def test_parser_ignores_comment_and_string_noise():
    note = "assert total == 0 and pytest.mark.skip"
    # pytest.skip("not a real skip")
    assert (
        total
        == expected
    )
