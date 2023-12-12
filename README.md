# IsPies

<div style="display:flex; justify-content:center;">
  <img src="./assets/images/logo.png" width="300"/>
</div>

## Description

This project aims to provide a multi-language set of useless tools to check if
a thing passed to them is a *pies* (from the Polish word for "dog") or not.
It is heavily inspired by a function in the [PKP Intercity ticketing system source code]
named `czyWybranoPsa` that in in its original form returned a boolean value
indicating if a passenger has selected a dog as their pet.

This boolean value was named

<div style="display:flex; justify-content:center;margin-top:-2rem;">
  <img src="./assets/images/ispies.png" width="300"/>
</div>

A legendary variable name that has
inspired many programmers to not be ashamed of including their mother tongue
in the code they write. Since then, the code has been refactored and this
milestone has been lost. [But the legend lives on].

## What constitutes a pies?

According to the old [New Athens Polish encyclopedia], a definition of a "horse"
proposed by Joachim Chmielowski was as follows:

> Horse: Everyone can see what a horse is

In case of a *pies*, the matter is a bit more complicated, especially if we think about it
in a context of an programming input. In this case, the essence of a *pies* may be idempotent,
but its form greatly varies on the method of its transmission.

For the purposes of this project, we will distinguish the following types of *pies*es:

### Text-based pies (TBP)

A TBP is a *pies* that is transmitted as a text. As text we will consider any
sequence of characters that can be represented as a string in a programming language.

This also includes any binary data that can be decoded as a string.

### Image-based pies (IBP)

An IBP is a *pies* that is transmitted as an image. This image may be of any format
that can be decoded by a computer and displayed on a screen.

Another special case of IBP is a TBP that is transmitted as an image. If such a situation occurs,
an initial IBP to TBP conversion must be performed before further processing.

### Sound-based pies (SBP)

An SBP is a *pies* that is transmitted as a sound. This sound may be of any format and its content
may include both human and non-human sounds.

In the case of a human sound, the input will be treated still as SBP, but converted to TBP
for further processing i.e. if the topic of the spoken text is indeed a *pies*.

For non-human sounds,
an obvious check will be performed to determine if its source is a well-functioning *pies*.

A special case of SBP is an IBP that is transmitted as a sound i.e. a spectrogram.
If such a situation occurs, the conversion to IBP must be performed before further processing.

## Project goal

The goal is to create a Docker image that will contain all the code and will
automatically identify if a given input is a *pies* or not. The user will be able
to pass the input in any form and the image will return a boolean value indicating
if the input is a *pies* or not.

## Contributing

If you want to contribute to this project, please read the [CONTRIBUTING.md](./CONTRIBUTING.md) file.

[PKP Intercity ticketing system source code]: https://bilet.intercity.pl/eic_js/zakup_biletu_plugin.js?ver=880871484
[But the legend lives on]: https://wykop.pl/wpis/30442309/kod-pkp-jest-zlotem-function-czywybranopsa-var-isp
[New Athens Polish encyclopedia]: https://en.wikipedia.org/wiki/Nowe_Ateny
