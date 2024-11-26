# RLARNDG (RustLangEs's Actually Random Generator)

A non-pseudo random number generator, based on images from cameras around the world, by taking their `M3u8`
source we grab random bytes from the image, and based on that we generate integers, booleans, colors and more.

To predict the number you should grab the camera source, know in what order are they, and the selected one,
that based on `unix` time and also knowing at what time a camera generated repeated content, and then know what
is on that specific camera at that specific time in the future thus real random.

# Usage

The random numbers are provided by API HTTP endpoints, so it can be implemented into applications in
a easier way. The program is pre-made to contain API-KEYS which you can buy making a donation, otherwise
you can make requests without API-KEY, but these are limited to 1 request every 30 seconds.

The program provides a front-end with it's documentation, you can self-host/modify the program as you wish
in fact you can self-host the back-end only and remove the paywall, but we use a paywall to fund the `rustlang-es`
project.

# Self-Hosting

The project provides a makefile with a dev recipe, the dev recipe requires you to have a `.env` file
with the `STRIPE_SECRET` and `DATABASE_URL`, the database URL should be a `postgres` URL.

Or to deploy it yourself we currently pull the image manually prior to making some CI/CD for the
`rustlang-es` VPS.

For a production build you only need the `STRIPE_SECRET` as the database is managed by the `compose` file.

# Missing features

Even tho it's a Spanish community, I personally use English to code, thus I made everything in English,
the project is missing an I18n implementation to add both languages, and the tests mentioned in the open issue.
