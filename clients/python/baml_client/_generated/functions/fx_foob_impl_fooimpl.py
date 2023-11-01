# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.0.1
# Generated Date: __DATE__
# Generated by: vbv

from ..._impl.deserializer import Deserializer
from ..clients.client_myclient import MyClient
from ..types.classes.cls_inputtype import InputType
from ..types.classes.cls_inputtype2 import InputType2
from ..types.classes.cls_outputtype import OutputType
from ..types.enums.enm_sentiment import Sentiment
from .fx_foob import BAMLFooB


# Impl: FooImpl
# Client: MyClient
# An implementation of .


__prompt_template = """\
A {arg.a}!!


{arg.a.c}

the rest of the prompt
no-tab
  tab1
    tab2
morespaces here
{arg.a.c} {arg.b} hi there
JSON:
{//BAML_CLIENT_REPLACE_ME_MAGIC_Sentiment//}
{//BAML_CLIENT_REPLACE_ME_MAGIC_output//}\
"""

__output_replacer = {
    "{//BAML_CLIENT_REPLACE_ME_MAGIC_output//}": """\
{	
    "sentiment": "Sentiment as string",	// this is a description
    "is_positive": bool
}\
"""
,
    "{//BAML_CLIENT_REPLACE_ME_MAGIC_Sentiment//}": """\
Sentiment:


\
"""

}

# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[OutputType](OutputType)  # type: ignore


@BAMLFooB.register_impl("FooImpl")
async def FooImpl(arg: InputType, /) -> OutputType:
    prompt = __prompt_template.format(arg=arg)
    response = await MyClient.run_prompt(prompt)
    return __deserializer.from_string(response.generated)
