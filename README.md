# Splash CLI (Rust Edition)
>  A simple, command line tool to download Unsplash wallpapers. It’s not intended to be anything particularly fancy — it just works. 

## Roadmap
The main goal is to replicate the functionality of [splash-cli][splash-cli]

### Project
- [-] Setup CI / CD 
  - [ ] Auto release (CD)
  - [x] Run tests and fmt 
- [ ] Add tests
- [ ] Centralize (as possible) error handling ([`quick_error`][quick-error]?)

### Features
- [x] Download random photos
- [x] Download photo of the day
- [ ] Manage preferences
- [ ] Image manipulation

- [ ] User authentication
  - [ ] Create / Edit collections
  - [ ] Create / Edit aliases
  - [ ] Like photos.

## Contributing Guide
Here some small tips about the project: 
All the **api** related methods goes under the `api::unsplash` and `api::models` modules.
To avoid spaghetti code and duplication create a set functions under `lib::utils::{category}` (ex: `lib::utils::photos` or `lib::utils::user` etc).

Any improvement or suggestion is welcome.


## Related
- [splash-cli][splash-cli]
- [Unsplash API](https://unsplash.com/developers) - Here you can read docs and register a test apikey for development.


[splash-cli]: https://github.com/splash-cli/splash-cli
[quick-error]: https://github.com/tailhook/quick-error
