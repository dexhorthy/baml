# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..types.classes.cls_conversation import Conversation
from ..types.classes.cls_improvedresponse import ImprovedResponse
from ..types.classes.cls_message import Message
from ..types.classes.cls_proposedmessage import ProposedMessage
from ..types.enums.enm_messagesender import MessageSender
from ..types.enums.enm_sentiment import Sentiment
from typing import Protocol, runtime_checkable


import typing

import pytest
from contextlib import contextmanager
from unittest import mock

ImplName = typing.Literal["v1", "v2"]

T = typing.TypeVar("T", bound=typing.Callable[..., typing.Any])
CLS = typing.TypeVar("CLS", bound=type)


IMaybePolishTextOutput = ImprovedResponse

@runtime_checkable
class IMaybePolishText(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: ProposedMessage

    Returns:
        ImprovedResponse
    """

    async def __call__(self, arg: ProposedMessage, /) -> ImprovedResponse:
        ...


class BAMLMaybePolishTextImpl:
    async def run(self, arg: ProposedMessage, /) -> ImprovedResponse:
        ...

class IBAMLMaybePolishText:
    def register_impl(
        self, name: ImplName
    ) -> typing.Callable[[IMaybePolishText], IMaybePolishText]:
        ...

    async def __call__(self, arg: ProposedMessage, /) -> ImprovedResponse:
        ...

    def get_impl(self, name: ImplName) -> BAMLMaybePolishTextImpl:
        ...

    @contextmanager
    def mock(self) -> typing.Generator[mock.AsyncMock, None, None]:
        """
        Utility for mocking the MaybePolishTextInterface.

        Usage:
            ```python
            # All implementations are mocked.

            async def test_logic() -> None:
                with baml.MaybePolishText.mock() as mocked:
                    mocked.return_value = ...
                    result = await MaybePolishTextImpl(...)
                    assert mocked.called
            ```
        """
        ...

    @typing.overload
    def test(self, test_function: T) -> T:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the MaybePolishTextInterface.

        Args:
            test_function : T
                The test function to be decorated.

        Usage:
            ```python
            # All implementations will be tested.

            @baml.MaybePolishText.test
            async def test_logic(MaybePolishTextImpl: IMaybePolishText) -> None:
                result = await MaybePolishTextImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, *, exclude_impl: typing.Iterable[ImplName]) -> pytest.MarkDecorator:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the MaybePolishTextInterface.

        Args:
            exclude_impl : Iterable[ImplName]
                The names of the implementations to exclude from testing.

        Usage:
            ```python
            # All implementations except "v1" will be tested.

            @baml.MaybePolishText.test(exclude_impl=["v1"])
            async def test_logic(MaybePolishTextImpl: IMaybePolishText) -> None:
                result = await MaybePolishTextImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, test_class: typing.Type[CLS]) -> typing.Type[CLS]:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the MaybePolishTextInterface.

        Args:
            test_class : Type[CLS]
                The test class to be decorated.

        Usage:
        ```python
        # All implementations will be tested in every test method.

        @baml.MaybePolishText.test
        class TestClass:
            def test_a(self, MaybePolishTextImpl: IMaybePolishText) -> None:
                ...
            def test_b(self, MaybePolishTextImpl: IMaybePolishText) -> None:
                ...
        ```
        """
        ...

BAMLMaybePolishText: IBAMLMaybePolishText
