# Bevy UI Stuff

I want to preface this by saying I don't have much experience with Bevy, and these things are just things I've thought about while working on my game jam entry - they're very much not demands. The stuff I have implemented may not be the most ideal either, I know that macros exist which would simplify the StyleBuilder but I haven't used macros yet.

## In this repo

- Rem and Em units - I think Rem is essential, but Em would be nice to have but it deals with inheretance and the use case is limited.
- Val::Calc()
- StyleBuilder - for making easily editable and shareable styles that affect both Style and other properties such as background_color.
- Button tweaks - component to determine if an action was a real click (down and up on the element) and reduces boilerplate, and component to easily control cursor. I'm not sure how well it fits into Bevy's design.

## Other things

Not in this repo

- Container queries - a way to apply styles that fit size criteria for viewport and container. Just not sure how it should look.
