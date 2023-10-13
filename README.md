# Angelspeech

This is a Rust library originally created to power the
(https://github.com/taernsietr/language_generator)[language_generator] project.
The code here was previously contained within that repo, but I separated them
for better modularity and maintainability.  

As this contains the \[possibly\] actually interesting code, any future features
will be added to this crate, then made accessibly in `language_generator`'s API.   

If anyone wants to use this crate as-is, either:

- clone both projects, alter `Cargo.toml` in `language_generator` to point to
  where this crate is on your device, then run `language_generator` as indicated
in its readme, or  

- clone this crate and import it locally, importing the struct
  `angelspeech::generator::text_generator::TextGenerator`.
