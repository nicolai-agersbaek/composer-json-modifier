use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::parse_handler::{ParseFile,ParseFileType};

#[derive(Debug, Serialize, Deserialize)]
pub struct ComposerJson {
    /// The name of the package. It consists of vendor name and project name, separated by `/`.
    ///
    /// **Examples:**
    ///
    /// - `monolog/monolog`
    /// - `igorw/event-source`
    ///
    /// The name must be lowercase and consist of words separated by `-`, `.` or `_`.
    /// The complete name should match `^[a-z0-9]([_.-]?[a-z0-9]+)*/[a-z0-9](([_.]|-{1,2})?[a-z0-9]+)*$`.
    ///
    /// The `name` property is required for published packages (libraries).
    ///
    /// **Note:** Before Composer version 2.0, a name could contain any character, including white spaces.
    ///
    /// Reference: [The composer.json schema (name)](https://getcomposer.org/doc/04-schema.md#name).
    pub name: String,

    /// A short description of the package. Usually this is one line long.
    ///
    /// Required for published packages (libraries).
    ///
    /// Reference: [The composer.json schema (description)](https://getcomposer.org/doc/04-schema.md#description).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The version of the package. In most cases this is not required and should be omitted (see below).
    ///
    /// This must follow the format of `X.Y.Z` or `vX.Y.Z` with an optional suffix of `-dev`, `-patch` (`-p`), `-alpha` (`-a`), `-beta` (`-b`) or `-RC`.
    /// The patch, alpha, beta and RC suffixes can also be followed by a number.
    ///
    /// **Examples:**
    ///
    /// - `1.0.0`
    /// - `1.0.2`
    /// - `1.1.0`
    /// - `0.2.5`
    /// - `1.0.0-dev`
    /// - `1.0.0-alpha3`
    /// - `1.0.0-beta2`
    /// - `1.0.0-RC5`
    /// - `v2.0.4-p1`
    ///
    /// Optional if the package repository can infer the version from somewhere, such as the VCS tag name in the VCS repository.
    /// In that case it is also recommended to omit it.
    ///
    /// **Note:** Packagist uses VCS repositories, so the statement above is very much true for
    /// Packagist as well. Specifying the version yourself will most likely end up creating problems
    /// at some point due to human error.
    ///
    /// Reference: [The composer.json schema (version)](https://getcomposer.org/doc/04-schema.md#version).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The type of the package. It defaults to `library`.
    ///
    /// Package types are used for custom installation logic. If you have a package that needs some special logic, you can define a custom type.
    /// This could be a `symfony-bundle`, a `wordpress-plugin` or a `typo3-cms-extension`.
    /// These types will all be specific to certain projects, and they will need to provide an installer capable of installing packages of that type.
    ///
    /// Out of the box, Composer supports four types:
    ///
    /// - **library:** This is the default. It will copy the files to `vendor`.
    /// - **project:** This denotes a project rather than a library.
    ///                For example application shells like the (Symfony standard edition)[https://github.com/symfony/symfony-standard],
    ///                CMSs like the (Silverstripe installer)[https://github.com/silverstripe/silverstripe-installer] or full fledged
    ///                applications distributed as packages.
    ///                This can for example be used by IDEs to provide listings of projects to initialize when creating a new workspace.
    /// - **metapackage:** An empty package that contains requirements and will trigger their installation,
    ///                    but contains no files and will not write anything to the filesystem. As such,
    ///                    it does not require a dist or source key to be installable.
    /// - **composer-plugin:** A package of type `composer-plugin` may provide an installer for other packages
    ///                        that have a custom type. Read more in the [dedicated article](https://getcomposer.org/doc/articles/custom-installers.md).
    ///
    /// Only use a custom type if you need custom logic during installation.
    /// It is recommended to omit this field and have it default to `library`.
    ///
    /// Reference: [The composer.json schema (type)](https://getcomposer.org/doc/04-schema.md#type).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<PackageType>,

    /// An array of keywords that the package is related to. These can be used for searching and filtering.
    ///
    /// **Examples:**
    ///
    /// - logging
    /// - events
    /// - database
    /// - redis
    /// - templating
    ///
    /// **Note:**
    /// Some special keywords trigger `composer require` without the `--dev` option to prompt users
    /// if they would like to add these packages to `require-dev` instead of `require`.
    /// These are: `dev`, `testing`, `static analysis`.
    ///
    /// Reference: [The composer.json schema (keywords)](https://getcomposer.org/doc/04-schema.md#keywords).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    /// A URL to the website of the project.
    ///
    /// Reference: [The composer.json schema (homepage)](https://getcomposer.org/doc/04-schema.md#homepage).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    /// A relative path to the readme document. Defaults to `README.md`.
    ///
    /// This is mainly useful for packages not on GitHub, as for GitHub packages,
    /// Packagist.org will use the readme API to fetch the one detected by GitHub.
    ///
    /// Reference: [The composer.json schema (readme)](https://getcomposer.org/doc/04-schema.md#readme).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,

    /// Release date of the version.
    ///
    /// Must be in `YYYY-MM-DD` or `YYYY-MM-DD HH:MM:SS` format.
    ///
    /// Reference: [The composer.json schema (time)](https://getcomposer.org/doc/04-schema.md#time).
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Should be an optional date-time!
    pub time: Option<String>,

    /// The license of the package. This can be either a string or an array of strings.
    ///
    /// The recommended notation for the most common licenses is (alphabetical):
    ///
    /// - Apache-2.0
    /// - BSD-2-Clause
    /// - BSD-3-Clause
    /// - BSD-4-Clause
    /// - GPL-2.0-only / GPL-2.0-or-later
    /// - GPL-3.0-only / GPL-3.0-or-later
    /// - LGPL-2.1-only / LGPL-2.1-or-later
    /// - LGPL-3.0-only / LGPL-3.0-or-later
    /// - MIT
    ///
    /// Optional, but it is highly recommended to supply this.
    /// More identifiers are listed at the (SPDX Open Source License Registry)[https://spdx.org/licenses/].
    ///
    /// **Note:**
    /// For closed-source software, you may use `"proprietary"` as the license identifier.
    ///
    /// An Example:
    ///
    /// ```json
    /// {
    ///     "license": "MIT"
    /// }
    /// ```
    ///
    /// For a package, when there is a choice between licenses ("disjunctive license"), multiple can be specified as an array.
    ///
    /// An Example for disjunctive licenses:
    ///
    /// ```json
    /// {
    ///     "license": [
    ///         "LGPL-2.1-only",
    ///         "GPL-3.0-or-later"
    ///     ]
    /// }
    /// ```
    ///
    /// Alternatively they can be separated with "or" and enclosed in parentheses;
    ///
    /// ```json
    /// {
    ///     "license": "(LGPL-2.1-only or GPL-3.0-or-later)"
    /// }
    /// ```
    ///
    /// Similarly, when multiple licenses need to be applied ("conjunctive license"), they should be separated with "and" and enclosed in parentheses.
    ///
    /// Reference: [The composer.json schema (license)](https://getcomposer.org/doc/04-schema.md#license).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<OneOrMany<String>>,

    /// The authors of the package. This is an array of objects.
    ///
    /// Each author object can have following properties:
    ///
    /// - **name**: The author's name. Usually their real name.
    /// - **email**: The author's email address.
    /// - **homepage**: URL to the author's website.
    /// - **role**: The author's role in the project (e.g. developer or translator)
    ///
    /// An example:
    ///
    /// ```json
    /// {
    ///     "authors": [
    ///         {
    ///             "name": "Nils Adermann",
    ///             "email": "naderman@naderman.de",
    ///             "homepage": "https://www.naderman.de",
    ///             "role": "Developer"
    ///         },
    ///         {
    ///             "name": "Jordi Boggiano",
    ///             "email": "j.boggiano@seld.be",
    ///             "homepage": "https://seld.be",
    ///             "role": "Developer"
    ///         }
    ///     ]
    /// }
    /// ```
    ///
    /// Optional, but highly recommended.
    ///
    /// Reference: [The composer.json schema (authors)](https://getcomposer.org/doc/04-schema.md#authors).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Author>>,

    /// Various information to get support about the project.
    ///
    /// Support information includes the following:
    /// - **email**: Email address for support.
    /// - **issues**: URL to the issue tracker.
    /// - **forum**: URL to the forum.
    /// - **wiki**: URL to the wiki.
    /// - **irc**: IRC channel for support, as irc://server/channel.
    /// - **source**: URL to browse or download the sources.
    /// - **docs**: URL to the documentation.
    /// - **rss**: URL to the RSS feed.
    /// - **chat**: URL to the chat channel.
    /// - **security**: URL to the vulnerability disclosure policy (VDP).
    ///
    /// An example:
    ///
    /// ```json
    /// {
    ///     "support": {
    ///         "email": "support@example.org",
    ///         "irc": "irc://irc.freenode.org/composer"
    ///     }
    /// }
    /// ```
    ///
    /// Reference: [The composer.json schema (support)](https://getcomposer.org/doc/04-schema.md#support).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<Support>,

    /// A list of URLs to provide funding to the package authors for maintenance and development of new functionality.
    ///
    /// Each entry consists of the following
    /// - **type**: The type of funding, or the platform through which funding can be provided, e.g. patreon, opencollective, tidelift or github.
    /// - **url**: URL to a website with details, and a way to fund the package.
    ///
    /// An example:
    ///
    /// {
    ///     "funding": [
    ///         {
    ///             "type": "patreon",
    ///             "url": "https://www.patreon.com/phpdoctrine"
    ///         },
    ///         {
    ///             "type": "tidelift",
    ///             "url": "https://tidelift.com/subscription/pkg/packagist-doctrine_doctrine-bundle"
    ///         },
    ///         {
    ///             "type": "other",
    ///             "url": "https://www.doctrine-project.org/sponsorship.html"
    ///         }
    ///     ]
    /// }
    ///
    /// Reference: [The composer.json schema (funding)](https://getcomposer.org/doc/04-schema.md#funding).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<Vec<Funding>>,

    /// Reference: [The composer.json schema (package links)](https://getcomposer.org/doc/04-schema.md#package-links).
    #[serde(flatten)]
    pub package_links: PackageLinks,

    /// Autoload mapping for a PHP autoloader.
    ///
    /// (`PSR-4`)[https://www.php-fig.org/psr/psr-4/] and (`PSR-0`)[http://www.php-fig.org/psr/psr-0/] auto-loading,
    /// `classmap` generation and `files` includes are supported.
    ///
    /// PSR-4 is the recommended way since it offers greater ease of use (no need to regenerate the autoloader when you add classes).
    ///
    /// Reference: [The composer.json schema (autoload)](https://getcomposer.org/doc/04-schema.md#autoload).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoload: Option<Autoload>,

    /// This section allows defining autoload rules for development purposes.
    ///
    /// Classes needed to run the test suite should not be included in the main autoload rules to avoid
    /// polluting the autoloader in production and when other people use your package as a dependency.
    ///
    /// Therefore, it is a good idea to rely on a dedicated path for your unit tests and to add it within the autoload-dev section.
    ///
    /// **Example:**
    ///
    /// ```json
    /// {
    ///     "autoload": {
    ///         "psr-4": { "MyLibrary\\": "src/" }
    ///     },
    ///     "autoload-dev": {
    ///         "psr-4": { "MyLibrary\\Tests\\": "tests/" }
    ///     }
    /// }
    /// ```
    ///
    /// Reference: [The composer.json schema (autoload-dev)](https://getcomposer.org/doc/04-schema.md#autoload-dev).
    #[serde(rename = "autoload-dev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoload_dev: Option<Autoload>, // root-only

    /// **DEPRECATED:**
    /// This is only present to support legacy projects, and all new code should preferably
    /// use auto-loading. As such it is a deprecated practice, but the feature itself will
    /// not likely disappear from Composer.
    ///
    /// A list of paths which should get appended to PHP's `include_path`.
    ///
    /// **Example:**
    ///
    /// ```json
    /// {
    ///     "include-path": ["lib/"]
    /// }
    /// ```
    ///
    /// Reference: [The composer.json schema (include-path)](https://getcomposer.org/doc/04-schema.md#include-path).
    #[deprecated]
    #[serde(rename = "target-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_path: Option<Vec<String>>,

    /// **DEPRECATED:**
    /// This is only present to support legacy PSR-0 style auto-loading, and all
    /// new code should preferably use PSR-4 without target-dir and projects using
    /// PSR-0 with PHP namespaces are encouraged to migrate to PSR-4 instead.
    ///
    /// Defines the installation target.
    ///
    /// In case the package root is below the namespace declaration you cannot
    /// autoload properly. `target-dir` solves this problem.
    ///
    /// An example is Symfony. There are individual packages for the components.
    /// The Yaml component is under `Symfony\Component\Yaml`. The package root is
    /// that Yaml directory. To make auto-loading possible, we need to make sure
    /// that it is not installed into `vendor/symfony/yaml`, but instead into
    /// `vendor/symfony/yaml/Symfony/Component/Yaml`, so that the autoloader can
    /// load it from `vendor/symfony/yaml`.
    ///
    /// To do that, autoload and target-dir are defined as follows:
    ///
    /// ```json
    /// {
    ///     "autoload": {
    ///         "psr-0": { "Symfony\\Component\\Yaml\\": "" }
    ///     },
    ///     "target-dir": "Symfony/Component/Yaml"
    /// }
    /// ```
    ///
    /// Reference: [The composer.json schema (target-dir)](https://getcomposer.org/doc/04-schema.md#target-dir).
    #[deprecated]
    #[serde(rename = "target-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_dir: Option<String>,

    /// This defines the default behavior for filtering packages by stability.
    /// This defaults to `stable`, so if you rely on a `dev` package, you should specify it in your file to avoid surprises.
    ///
    /// All versions of each package are checked for stability, and those that are less stable
    /// than the `minimum-stability` setting will be ignored when resolving your project dependencies.
    /// (Note that you can also specify stability requirements on a per-package basis using stability
    /// flags in the version constraints that you specify in a `require` block
    /// (see (package links)[https://getcomposer.org/doc/04-schema.md#package-links] for more details).
    ///
    /// Available options (in order of stability) are:
    /// - dev
    /// - alpha
    /// - beta
    /// - RC
    /// - stable
    ///
    /// Reference: [The composer.json schema (minimum stability)](https://getcomposer.org/doc/04-schema.md#minimum-stability).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_stability: Option<Stability>,

    /// When this is enabled, Composer will prefer more stable packages over unstable ones when finding
    /// compatible stable packages is possible.
    /// If you require a dev version or only alphas are available for a package, those will still be
    /// selected granted that the minimum-stability allows for it.
    ///
    /// Use `"prefer-stable": true` to enable.
    ///
    /// Reference: [The composer.json schema (prefer stable)](https://getcomposer.org/doc/04-schema.md#prefer-stable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_stable: Option<bool>, // root-only

    /// Custom package repositories to use.
    ///
    /// By default Composer only uses the packagist repository.
    /// By specifying repositories you can get packages from elsewhere.
    ///
    /// Repositories are not resolved recursively.
    /// You can only add them to your main `composer.json`.
    /// Repository declarations of dependencies' `composer.json`s are ignored.
    ///
    /// The following repository types are supported:
    ///
    /// - **composer**: A Composer repository is a `packages.json` file served via the network (HTTP, FTP, SSH),
    ///                 that contains a list of `composer.json` objects with additional `dist` and/or `source` information.
    ///                 The `packages.json` file is loaded using a PHP stream.
    ///                 You can set extra options on that stream using the `options` parameter.
    /// - **vcs**: The version control system repository can fetch packages from git, svn, fossil and hg repositories.
    /// - **package**: If you depend on a project that does not have any support for Composer whatsoever
    ///                you can define the package inline using a `package` repository.
    ///                You basically inline the `composer.json` object.
    ///
    /// For more information on any of these, see (Repositories)[https://getcomposer.org/doc/05-repositories.md].
    ///
    /// **Example:**
    ///
    /// ```json
    /// {
    ///     "repositories": [
    ///         {
    ///             "type": "composer",
    ///             "url": "https://packages.example.com"
    ///         },
    ///         {
    ///             "type": "composer",
    ///             "url": "https://packages.example.com",
    ///             "options": {
    ///                 "ssl": {
    ///                     "verify_peer": "true"
    ///                 }
    ///             }
    ///         },
    ///         {
    ///             "type": "vcs",
    ///             "url": "https://github.com/Seldaek/monolog"
    ///         },
    ///         {
    ///             "type": "package",
    ///             "package": {
    ///                 "name": "smarty/smarty",
    ///                 "version": "3.1.7",
    ///                 "dist": {
    ///                     "url": "https://www.smarty.net/files/Smarty-3.1.7.zip",
    ///                     "type": "zip"
    ///                 },
    ///                 "source": {
    ///                     "url": "https://smarty-php.googlecode.com/svn/",
    ///                     "type": "svn",
    ///                     "reference": "tags/Smarty_3_1_7/distribution/"
    ///                 }
    ///             }
    ///         }
    ///     ]
    /// }
    /// ```
    ///
    /// **Note:**
    /// Order is significant here. When looking for a package, Composer will look from the first to
    /// the last repository, and pick the first match.
    /// By default Packagist is added last which means that custom repositories can override packages from it.
    ///
    /// Using JSON object notation is also possible.
    /// However, JSON key/value pairs are to be considered unordered so consistent behaviour cannot be guaranteed.
    ///
    /// ```json
    /// {
    ///     "repositories": {
    ///         "foo": {
    ///             "type": "composer",
    ///             "url": "https://packages.foo.com"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Reference: [The composer.json schema (repositories)](https://getcomposer.org/doc/04-schema.md#repositories).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<Repository>>, // root-only

    /// A set of configuration options. It is only used for projects. See Config for a description of each individual option.
    ///
    /// Reference: [The composer.json schema (config)](https://getcomposer.org/doc/04-schema.md#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>, // root-only

    /// Composer allows you to hook into various parts of the installation process through the use of scripts.
    ///
    /// See [Scripts](https://getcomposer.org/doc/articles/scripts.md) for events details and examples.
    ///
    /// Reference: [The composer.json schema (scripts)](https://getcomposer.org/doc/04-schema.md#scripts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Scripts>, // root-only

    /// Arbitrary extra data for consumption by `scripts`.
    ///
    /// This can be virtually anything. To access it from within a script event handler, you can do:
    ///
    /// ```php
    /// $extra = $event->getComposer()->getPackage()->getExtra();
    /// ```
    ///
    /// Reference: [The composer.json schema (extra)](https://getcomposer.org/doc/04-schema.md#extra).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>, // root-only

    /// A set of files that should be treated as binaries and made available into the `bin-dir` (from config).
    ///
    /// See [Vendor Binaries](https://getcomposer.org/doc/articles/vendor-binaries.md) for more details.
    ///
    /// Reference: [The composer.json schema (bin)](https://getcomposer.org/doc/04-schema.md#bin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Vec<String>>,

    /// A set of options for creating package archives.
    ///
    /// The following options are supported:
    ///
    /// - **name**: Allows configuring base name for archive.
    ///             By default (if not configured, and `--file` is not passed as command-line argument),
    ///             `preg_replace('#[^a-z0-9-_]#i', '-', name)` is used.
    ///
    /// **Example:**
    ///
    /// ```json
    /// {
    ///     "name": "org/strangeName",
    ///     "archive": {
    ///         "name": "Strange_name"
    ///     }
    /// }
    /// ```
    ///
    /// - **exclude**: Allows configuring a list of patterns for excluded paths. The pattern syntax
    ///                matches .gitignore files. A leading exclamation mark (!) will result in any
    ///                matching files to be included even if a previous pattern excluded them.
    ///                A leading slash will only match at the beginning of the project relative path.
    ///                An asterisk will not expand to a directory separator.
    ///
    /// **Example:**
    ///
    /// ```json
    /// {
    ///     "archive": {
    ///         "exclude": ["/foo/bar", "baz", "/*.test", "!/foo/bar/baz"]
    ///     }
    /// }
    /// ```
    ///
    /// The example will include `/dir/foo/bar/file`, `/foo/bar/baz`, `/file.php`, `/foo/my.test` but
    /// it will exclude `/foo/bar/any`, `/foo/baz`, and `/my.test`.
    ///
    /// Reference: [The composer.json schema (archive)](https://getcomposer.org/doc/04-schema.md#archive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Archive>,

    /// Indicates whether this package has been abandoned.
    ///
    /// It can be boolean or a package name/URL pointing to a recommended alternative.
    ///
    /// **Examples:**
    ///
    /// Use `"abandoned": true` to indicate this package is abandoned.
    /// Use `"abandoned": "monolog/monolog"` to indicate this package is abandoned,
    /// and that the recommended alternative is `monolog/monolog`.
    ///
    /// Defaults to `false`.
    ///
    /// Reference: [The composer.json schema (abandoned)](https://getcomposer.org/doc/04-schema.md#abandoned).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned: Option<Abandoned>,

    /// A list of regex patterns of branch names that are non-numeric (e.g. "latest" or something),
    /// that will NOT be handled as feature branches.
    /// This is an array of strings.
    ///
    /// If you have non-numeric branch names, for example like "latest", "current", "latest-stable"
    /// or something, that do not look like a version number, then Composer handles such branches as
    /// feature branches.
    /// This means it searches for parent branches, that look like a version or ends at special
    /// branches (like master), and the root package version number becomes the version of the
    /// parent branch or at least master or something.
    ///
    /// To handle non-numeric named branches as versions instead of searching for a parent branch
    /// with a valid version or special branch name like master, you can set patterns for branch
    /// names that should be handled as dev version branches.
    ///
    /// This is really helpful when you have dependencies using "self.version", so that not
    /// dev-master, but the same branch is installed (in the example: latest-testing).
    ///
    /// **An example:**
    ///
    /// If you have a testing branch, that is heavily maintained during a testing phase
    /// and is deployed to your staging environment, normally `composer show -s` will give
    /// you `versions : * dev-master`.
    ///
    /// If you configure `latest-.*` as a pattern for non-feature-branches like this:
    ///
    /// ```json
    /// {
    ///     "non-feature-branches": ["latest-.*"]
    /// }
    /// ```
    ///
    /// Then `composer show -s` will give you `versions : * dev-latest-testing`.
    ///
    /// Reference: [The composer.json schema (non-feature-branches)](https://getcomposer.org/doc/04-schema.md#non-feature-branches).
    #[serde(rename = "non-feature-branches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_feature_branches: Option<Vec<String>>,
}

impl ParseFile for ComposerJson {
    fn parse_file_type() -> ParseFileType {
        ParseFileType::ComposerJson
    }
}

/// Marks a field as only available in the root-level `composer.json` file.
///
/// The root package is the package defined by the `composer.json` at the root
/// of your project.
/// It is the main `composer.json` that defines your project requirements.
///
/// Certain fields only apply when in the root package context.
/// One example of this is the `config` field. Only the root package can
/// define configuration. The config of dependencies is ignored.
/// This makes the `config` field `root-only`.
///
/// **Note:**
/// A package can be the root package or not, depending on the context.
/// For example, if your project depends on the `monolog` library,
/// your project is the root package.
/// However, if you clone `monolog` from GitHub in order to fix a bug
/// in it, then `monolog` is the root package.
///
/// Reference: [Root Package](https://getcomposer.org/doc/04-schema.md#root-package).
trait RootOnly {}

#[derive(Debug, Serialize, Deserialize)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

/// The type of the package. It defaults to `library`.
///
/// Package types are used for custom installation logic. If you have a package that needs some special logic, you can define a custom type.
/// This could be a `symfony-bundle`, a `wordpress-plugin` or a `typo3-cms-extension`.
/// These types will all be specific to certain projects, and they will need to provide an installer capable of installing packages of that type.
///
/// Out of the box, Composer supports four types:
///
/// - **library:** This is the default. It will copy the files to `vendor`.
/// - **project:** This denotes a project rather than a library.
///                For example application shells like the (Symfony standard edition)[https://github.com/symfony/symfony-standard],
///                CMSs like the (Silverstripe installer)[https://github.com/silverstripe/silverstripe-installer] or full fledged
///                applications distributed as packages.
///                This can for example be used by IDEs to provide listings of projects to initialize when creating a new workspace.
/// - **metapackage:** An empty package that contains requirements and will trigger their installation,
///                    but contains no files and will not write anything to the filesystem. As such,
///                    it does not require a dist or source key to be installable.
/// - **composer-plugin:** A package of type `composer-plugin` may provide an installer for other packages
///                        that have a custom type. Read more in the [dedicated article](https://getcomposer.org/doc/articles/custom-installers.md).
///
/// Only use a custom type if you need custom logic during installation.
/// It is recommended to omit this field and have it default to `library`.
///
/// Reference: [The composer.json schema (type)](https://getcomposer.org/doc/04-schema.md#type).
#[derive(Debug, Serialize, Deserialize)]
pub enum PackageType {
    /// This is the default. It will copy the files to `vendor`.
    #[serde(rename = "library")]
    Library,

    /// This denotes a project rather than a library.
    /// For example application shells like the (Symfony standard edition)[https://github.com/symfony/symfony-standard],
    /// CMSs like the (Silverstripe installer)[https://github.com/silverstripe/silverstripe-installer] or full fledged
    /// applications distributed as packages.
    /// This can for example be used by IDEs to provide listings of projects to initialize when creating a new workspace.
    #[serde(rename = "project")]
    Project,

    /// An empty package that contains requirements and will trigger their installation,
    /// but contains no files and will not write anything to the filesystem. As such,
    /// it does not require a dist or source key to be installable.
    #[serde(rename = "metapackage")]
    Metapackage,

    /// A package of type `composer-plugin` may provide an installer for other packages
    /// that have a custom type. Read more in the [dedicated article](https://getcomposer.org/doc/articles/custom-installers.md).
    #[serde(rename = "composer-plugin")]
    ComposerPlugin,

    /// Custom package type.
    /// Only use a custom type if you need custom logic during installation.
    Custom(String),
}

/// All of the following take an object which maps package names to versions of the package via version constraints.
/// Read more about versions [here](https://getcomposer.org/doc/articles/versions.md).
///
/// **Example:**
///
/// ```json
/// {
///     "require": {
///         "monolog/monolog": "1.0.*"
///     }
/// }
/// ```
/// All links are optional fields.
///
/// `require` and `require-dev` additionally support _stability flags_ ([root-only](https://getcomposer.org/doc/04-schema.md#root-package)).
/// They take the form _"constraint@stability flag"_.
/// These allow you to further restrict or expand the stability of a package beyond the scope of the [minimum-stability](https://getcomposer.org/doc/04-schema.md#minimum-stability) setting.
/// You can apply them to a constraint, or apply them to an empty _constraint_ if you want to allow unstable packages of a dependency for example.
///
/// **Example:**
///
/// ```json
/// {
///     "require": {
///         "monolog/monolog": "1.0.*@beta",
///         "acme/foo": "@dev"
///     }
/// }
/// ```
/// If one of your dependencies has a dependency on an unstable package you need to explicitly require it as well, along with its sufficient stability flag.
///
/// **Example:**
///
/// Assuming `doctrine/doctrine-fixtures-bundle` requires `"doctrine/data-fixtures": "dev-master"` then
/// inside the root composer.json you need to add the second line below to allow dev releases for the `doctrine/data-fixtures` package:
///
/// ```json
/// {
///     "require": {
///         "doctrine/doctrine-fixtures-bundle": "dev-master",
///         "doctrine/data-fixtures": "@dev"
///     }
/// }
/// ```
/// `require` and `require-dev` additionally support explicit references (i.e. commit) for dev versions
/// to make sure they are locked to a given state, even when you run update.
/// These only work if you explicitly require a dev version and append the reference with `#<ref>`.
/// This is also a [root-only](https://getcomposer.org/doc/04-schema.md#root-package) feature and will be ignored in dependencies.
///
/// **Example:**
///
/// ```json
/// {
///     "require": {
///         "monolog/monolog": "dev-master#2eb0c0978d290a1c45346a1955188929cb4e5db7",
///         "acme/foo": "1.0.x-dev#abc123"
///     }
/// }
/// ```
/// **Note:** This feature has severe technical limitations, as the composer.json metadata will still
/// be read from the branch name you specify before the hash.
/// You should therefore only use this as a temporary solution during development to remediate transient
/// issues, until you can switch to tagged releases.
/// The Composer team does not actively support this feature and will not accept bug reports related to it.
///
/// It is also possible to inline-alias a package constraint so that it matches a constraint that it
/// otherwise would not. For more information [see the aliases article](https://getcomposer.org/doc/articles/aliases.md).
///
/// `require` and `require-dev` also support references to specific PHP versions and PHP extensions your project needs to run successfully.
///
/// **Example:**
///
/// ```json
/// {
///     "require": {
///         "php": ">=7.4",
///         "ext-mbstring": "*"
///     }
/// }
/// ```
///
/// **Note:** It is important to list PHP extensions your project requires. Not all PHP installations are created equal: some may miss extensions you may consider as standard (such as ext-mysqli which is not installed by default in Fedora/CentOS minimal installation systems). Failure to list required PHP extensions may lead to a bad user experience: Composer will install your package without any errors but it will then fail at run-time. The composer show --platform command lists all PHP extensions available on your system. You may use it to help you compile the list of extensions you use and require. Alternatively you may use third party tools to analyze your project for the list of extensions used.
///
/// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#package-links) for details.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageLinks {
    /// Map of packages required by this package. The package will not be installed unless those requirements can be met.
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#require) for details.
    pub require: HashMap<String, String>,

    /// Map of packages required for developing this package, or running tests, etc.
    /// The dev requirements of the root package are installed by default.
    /// Both `install` or `update` support the `--no-dev` option that prevents dev dependencies from being installed.
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#require-dev) for details.
    #[serde(rename = "require-dev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_dev: Option<HashMap<String, String>>, // root-only

    /// Map of packages that conflict with this version of this package.
    /// They will not be allowed to be installed together with your package.
    ///
    /// Note that when specifying ranges like `<1.0 >=1.1` in a conflict link, this will state a conflict
    /// with all versions that are less than 1.0 _and_ equal or newer than 1.1 at the same time,
    /// which is probably not what you want. You probably want to go for `<1.0 || >=1.1` in this case.
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#conflict) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<HashMap<String, String>>,

    /// Map of packages that are replaced by this package.
    /// This allows you to fork a package, publish it under a different name with its own version numbers,
    /// while packages requiring the original package continue to work with your fork because it
    /// replaces the original package.
    ///
    /// This is also useful for packages that contain sub-packages, for example the main symfony/symfony package
    /// contains all the Symfony Components which are also available as individual packages.
    /// If you require the main package it will automatically fulfill any requirement of one of the
    /// individual components, since it replaces them.
    ///
    /// Caution is advised when using replace for the sub-package purpose explained above.
    /// You should then typically only replace using `self.version` as a version constraint, to make
    /// sure the main package only replaces the sub-packages of that exact version, and not any other
    /// version, which would be incorrect.
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#replace) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<HashMap<String, String>>,

    /// Map of packages that are provided by this package.
    /// This is mostly useful for implementations of common interfaces.
    /// A package could depend on some virtual package e.g. `psr/logger-implementation`, any library
    /// that implements this logger interface would list it in provide.
    /// Implementors can then be [found on Packagist.org](https://packagist.org/providers/psr/log-implementation).
    ///
    /// Using `provide` with the name of an actual package rather than a virtual one implies that the
    /// code of that package is also shipped, in which case `replace` is generally a better choice.
    /// A common convention for packages providing an interface and relying on other packages to provide
    /// an implementation (for instance the PSR interfaces) is to use a `-implementation` suffix for
    /// the name of the virtual package corresponding to the interface package.
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#provide) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provide: Option<HashMap<String, String>>,

    /// Suggested packages that can enhance or work well with this package.
    /// These are informational and are displayed after the package is installed, to give your users
    /// a hint that they could add more packages, even though they are not strictly required.
    ///
    /// The format is like package links above, except that the values are free text and not version constraints.
    ///
    /// **Example:**
    /// ```json
    /// {
    ///     "suggest": {
    ///         "monolog/monolog": "Allows more advanced logging of the application flow",
    ///         "ext-xml": "Needed to support XML format in class Foo"
    ///     }
    /// }
    /// ```
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#suggest) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggest: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Various information to get support about the project.
///
/// Support information includes the following:
/// - **email**: Email address for support.
/// - **issues**: URL to the issue tracker.
/// - **forum**: URL to the forum.
/// - **wiki**: URL to the wiki.
/// - **irc**: IRC channel for support, as irc://server/channel.
/// - **source**: URL to browse or download the sources.
/// - **docs**: URL to the documentation.
/// - **rss**: URL to the RSS feed.
/// - **chat**: URL to the chat channel.
/// - **security**: URL to the vulnerability disclosure policy (VDP).
///
/// An example:
///
/// ```json
/// {
///     "support": {
///         "email": "support@example.org",
///         "irc": "irc://irc.freenode.org/composer"
///     }
/// }
/// ```
///
/// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#support) for details.
#[derive(Debug, Serialize, Deserialize)]
pub struct Support {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irc: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
}

/// A funding entry consists of the following
/// - **type**: The type of funding, or the platform through which funding can be provided, e.g. patreon, opencollective, tidelift or github.
/// - **url**: URL to a website with details, and a way to fund the package.
///
/// An example:
///
/// ```json
/// {
///     "type": "patreon",
///     "url": "https://www.patreon.com/phpdoctrine"
/// }
/// ```
///
/// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#funding) for details.
#[derive(Debug, Serialize, Deserialize)]
pub struct Funding {
    #[serde(rename = "type")]
    pub platform: String,
    pub url: String,
}

/// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#autoload) for details.
#[derive(Debug, Serialize, Deserialize)]
pub struct Autoload {
    /// Under the `psr-4` key you define a mapping from namespaces to paths, relative to the package root.
    /// When autoloading a class like `Foo\\Bar\\Baz` a namespace prefix `Foo\\` pointing to a directory `src/`
    /// means that the autoloader will look for a file named `src/Bar/Baz.php` and include it if present.
    /// Note that as opposed to the older PSR-0 style, the prefix (`Foo\\`) is **not** present in the file path.
    ///
    /// Namespace prefixes must end in `\\` to avoid conflicts between similar prefixes.
    /// For example `Foo` would match classes in the `FooBar` namespace so the trailing backslashes
    /// solve the problem: `Foo\\` and `FooBar\\` are distinct.
    ///
    /// The PSR-4 references are all combined, during install/update, into a single key => value array
    /// which may be found in the generated file `vendor/composer/autoload_psr4.php`.
    ///
    /// Example:
    ///
    /// ```json
    /// {
    ///     "Monolog\\": "src/",
    ///     "Vendor\\Namespace\\": ""
    /// }
    /// ```
    ///
    /// If you need to search for a same prefix in multiple directories, you can specify them as an array as such:
    ///
    /// ```json
    /// {
    ///     "Monolog\\": ["src/", "lib/"]
    /// }
    /// ```
    ///
    /// If you want to have a fallback directory where any namespace will be looked for, you can use an empty prefix like:
    ///
    /// ```json
    /// {
    ///     "": "src/"
    /// }
    /// ```
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#psr-4) for details.
    #[serde(rename = "psr-4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psr_4: Option<HashMap<String, String>>,

    /// Under the `psr-0` key you define a mapping from namespaces to paths, relative to the package root.
    /// Note that this also supports the PEAR-style non-namespaced convention.
    ///
    /// Please note namespace declarations should end in `\\` to make sure the autoloader responds exactly.
    /// For example `Foo` would match in `FooBar` so the trailing backslashes solve the problem: `Foo\\` and `FooBar\\` are distinct.
    ///
    /// The PSR-0 references are all combined, during install/update, into a single key => value array
    /// which may be found in the generated file `vendor/composer/autoload_namespaces.php`.
    ///
    /// **Example:**
    ///
    /// ```json
    /// {
    ///      "Monolog\\": "src/",
    ///      "Vendor\\Namespace\\": "src/",
    ///      "Vendor_Namespace_": "src/"
    /// }
    /// ```
    ///
    /// If you need to search for a same prefix in multiple directories, you can specify them as an array as such:
    ///
    /// ```json
    /// {
    ///     "Monolog\\": ["src/", "lib/"]
    /// }
    /// ```
    ///
    /// The PSR-0 style is not limited to namespace declarations only but may be specified right down to the class level. This can be useful for libraries with only one class in the global namespace. If the php source file is also located in the root of the package, for example, it may be declared like this:
    ///
    /// ```json
    /// {
    ///     "UniqueGlobalClass": ""
    /// }
    /// ```
    ///
    /// If you want to have a fallback directory where any namespace can be, you can use an empty prefix like:
    ///
    /// ```json
    /// {
    ///     "": "src/"
    /// }
    /// ```
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#psr-0) for details.
    #[serde(rename = "psr-0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psr_0: Option<HashMap<String, String>>,

    /// The `classmap` references are all combined, during install/update, into a single key => value array
    /// which may be found in the generated file `vendor/composer/autoload_classmap.php`.
    /// This map is built by scanning for classes in all `.php` and `.inc` files in the given directories/files.
    ///
    /// You can use the classmap generation support to define autoloading for all libraries that do not follow PSR-0/4.
    /// To configure this you specify all directories or files to search for classes.
    ///
    /// **Example:**
    ///
    /// ```json
    ///     ["src/", "lib/", "Something.php"]
    /// ```
    ///
    /// Wildcards (`*`) are also supported in a classmap paths, and expand to match any directory name:
    ///
    /// **Example:**
    ///
    /// ```json
    ///     ["src/addons/*/lib/", "3rd-party/*", "Something.php"]
    /// ```
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#classmap) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classmap: Option<Vec<String>>,

    /// If you want to require certain files explicitly on every request then you can use the `files` autoloading mechanism.
    /// This is useful if your package includes PHP functions that cannot be autoloaded by PHP.
    ///
    /// **Example:**
    ///
    /// ```json
    ///     ["src/MyLibrary/functions.php"]
    /// ```
    ///
    /// Files autoload rules are included whenever `vendor/autoload.php` is included, right after the
    /// autoloader is registered.
    /// The order of inclusion depends on package dependencies so that if package A depends on B,
    /// files in package B will be included first to ensure package B is fully initialized and ready
    /// to be used when files from package A are included.
    ///
    /// If two packages have the same amount of dependents or no dependencies, the order is alphabetical.
    ///
    /// Files from the root package are always loaded last, and you cannot use files autoloading yourself
    /// to override functions from your dependencies.
    /// If you want to achieve that we recommend you include your own functions before
    /// including Composer's `vendor/autoload.php`.
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#files) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,

    /// If you want to exclude some files or folders from the classmap you can use the `exclude-from-classmap` property.
    /// This might be useful to exclude test classes in your live environment, for example, as those
    /// will be skipped from the classmap even when building an optimized autoloader.
    ///
    /// The classmap generator will ignore all files in the paths configured here.
    /// The paths are absolute from the package root directory (i.e. composer.json location),
    /// and support `*` to match anything but a slash, and `**` to match anything.
    /// `**` is implicitly added to the end of the paths.
    ///
    /// **Example:**
    ///
    /// ```json
    /// ["/Tests/", "/test/", "/tests/"]
    /// ```
    ///
    /// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#exclude-from-classmap) for details.
    #[serde(rename = "exclude-from-classmap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_classmap: Option<Vec<String>>,
}

/// Defines the stability of a package.
///
/// Available options (in order of stability) are:
/// - dev
/// - alpha
/// - beta
/// - RC
/// - stable
///
/// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#minimum-stability) for details.
#[derive(Debug, Serialize, Deserialize)]
pub enum Stability {
    #[serde(rename = "dev")]
    Dev,

    #[serde(rename = "alpha")]
    Alpha,

    #[serde(rename = "beta")]
    Beta,

    RC,

    #[serde(rename = "stable")]
    Stable,
}

/// A repository is a package source. It's a list of packages/versions.
/// Composer will look in all your repositories to find the packages your project requires.
///
/// By default, only the Packagist.org repository is registered in Composer.
/// You can add more repositories to your project by declaring them in `composer.json`.
///
/// Repositories are only available to the root package and the repositories
/// defined in your dependencies will not be loaded.
/// Read the [FAQ entry](https://getcomposer.org/doc/faqs/why-cant-composer-load-repositories-recursively.md) if you want to learn why.
///
/// When resolving dependencies, packages are looked up from repositories from
/// top to bottom, and by default, as soon as a package is found in one, Composer
/// stops looking in other repositories.
/// Read the [repository priorities](https://getcomposer.org/doc/articles/repository-priorities.md)
/// article for more details and to see how to change this behavior.
///
/// Reference: [Repository](https://getcomposer.org/doc/05-repositories.md#repository).
#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename = "type")]
    pub repository_type: RepositoryType,

    pub url: String,
}

/// Valid type for a repository.
///
/// Possible values are:
/// - [composer](https://getcomposer.org/doc/05-repositories.md#composer)
/// - [vcs](https://getcomposer.org/doc/05-repositories.md#vcs)
/// - [package](https://getcomposer.org/doc/05-repositories.md#package-2)
/// - [artifact](https://getcomposer.org/doc/05-repositories.md#artifact)
/// - [path](https://getcomposer.org/doc/05-repositories.md#path)
///
/// See: [Repositories](https://getcomposer.org/doc/05-repositories.md).
#[derive(Debug, Serialize, Deserialize)]
pub enum RepositoryType {
    /// The main repository type is the `composer` repository.
    /// It uses a single `packages.json` file that contains all of the package metadata.
    ///
    /// This is also the repository type that packagist uses.
    /// To reference a `composer` repository, supply the path before the `packages.json` file.
    /// In the case of packagist, that file is located at `/packages.json`, so the URL of the
    /// repository would be `repo.packagist.org`.
    /// For `example.org/packages.json` the repository URL would be `example.org`.
    ///
    /// ```json
    /// {
    ///     "repositories": [
    ///         {
    ///             "type": "composer",
    ///             "url": "https://example.org"
    ///         }
    ///     ]
    /// }
    /// ```
    ///
    /// Reference: [Repositories (Composer)](https://getcomposer.org/doc/05-repositories.md#composer).
    #[serde(rename = "composer")]
    Composer,

    /// VCS stands for version control system.
    /// This includes versioning systems like git, svn, fossil or hg.
    /// Composer has a repository type for installing packages from these systems.
    ///
    /// Reference: [Repositories (VCS)](https://getcomposer.org/doc/05-repositories.md#vcs).
    #[serde(rename = "vcs")]
    VCS,

    /// If you want to use a project that does not support Composer through any of the
    /// means above, you still can define the package yourself by using a `package` repository.
    ///
    /// Basically, you define the same information that is included in the `composer`
    /// repository's `packages.json`, but only for a single package.
    /// Again, the minimum required fields are `name`, `version`, and either of `dist` or `source`.
    ///
    /// Reference: [Repositories (Package)](https://getcomposer.org/doc/05-repositories.md#package-2).
    #[serde(rename = "package")]
    Package,

    /// There are some cases, when there is no ability to have one of the previously
    /// mentioned repository types online, even the VCS one.
    /// A typical example could be cross-organisation library exchange through build
    /// artifacts. Of course, most of the time these are private. To use these archives
    /// as-is, one can use a repository of type artifact with a folder containing ZIP
    /// or TAR archives of those private packages:
    ///
    /// ```json
    /// {
    ///     "repositories": [
    ///         {
    ///             "type": "artifact",
    ///             "url": "path/to/directory/with/zips/"
    ///         }
    ///     ],
    ///     "require": {
    ///         "private-vendor-one/core": "15.6.2",
    ///         "private-vendor-two/connectivity": "*",
    ///         "acme-corp/parser": "10.3.5"
    ///     }
    /// }
    /// ```
    ///
    /// Reference: [Repositories (Artifact)](https://getcomposer.org/doc/05-repositories.md#artifact).
    #[serde(rename = "artifact")]
    Artifact,

    /// In addition to the artifact repository, you can use the path one, which allows
    /// you to depend on a local directory, either absolute or relative.
    /// This can be especially useful when dealing with monolithic repositories.
    ///
    /// Reference: [Repositories (Path)](https://getcomposer.org/doc/05-repositories.md#path).
    #[serde(rename = "path")]
    Path,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PackageSourceConfig {
    #[serde(rename = "dist")]
    Dist,

    #[serde(rename = "source")]
    Source,

    #[serde(rename = "auto")]
    Auto,
}

type Host = String;

type GithubHost = Host;

type GitlabHost = Host;

type GitlabToken = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum GitlabTokenConfig {
    Simple(HashMap<GitlabHost, GitlabToken>),
    Detailed(HashMap<GitlabHost, GitlabTokenDetails>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitlabTokenDetails {
    username: String,
    token: GitlabToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GitProtocol {
    #[serde(rename = "git")]
    Git,

    #[serde(rename = "http")]
    Http,

    #[serde(rename = "https")]
    Https,
}

type BitbucketHost = Host;

#[derive(Debug, Serialize, Deserialize)]
pub struct BitbucketOauth {
    #[serde(rename = "consumer-key")]
    pub consumer_key: String,

    #[serde(rename = "consumer-secret")]
    pub consumer_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicAuth {
    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "password")]
    pub password: String,
}

pub type HttpBasicAuth = HashMap<Host, BasicAuth>;
pub type PlatformPackage = String;
pub type Version = String;
pub type PlatformConstraint = String;

//#[derive(Debug, Serialize, Deserialize)]
//pub enum PlatformConstraint {
    //Version(Version),
    //Hide(bool),
//}

#[derive(Debug, Serialize, Deserialize)]
pub enum BinaryCompatibility {
    #[serde(rename = "auto")]
    Auto,

    #[serde(rename = "full")]
    Full,

    #[serde(rename = "proxy")]
    Proxy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DiscardChangesMode {
    #[serde(rename = "stash")]
    Stash,

    Toggle(bool),
}

type ArchiveFormat = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum PlatformCheckMode {
    #[serde(rename = "php-only")]
    PhpOnly,

    Toggle(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PreferredInstall {
    #[serde(rename = "dist")]
    Dist,

    Map(HashMap<String, PackageSourceConfig>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The timeout in seconds for process executions, defaults to 300 (5 minutes).
    /// The duration processes like git clones can run before Composer assumes they died out.
    /// You may need to make this higher if you have a slow connection or huge vendors.
    ///
    /// To disable the process timeout on a custom command under `scripts`, a static helper is available:
    ///
    /// ```json
    /// {
    ///     "scripts": {
    ///         "test": [
    ///             "Composer\\Config::disableProcessTimeout",
    ///             "phpunit"
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Reference: [Config (process-timeout)](https://getcomposer.org/doc/06-config.md#process-timeout).
    #[serde(rename = "process-timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    process_timeout: Option<u32>,

    /// Defaults to `{}` which does not allow any plugins to be loaded.
    ///
    /// As of Composer 2.2.0, the `allow-plugins` option adds a layer of security allowing you
    /// to restrict which Composer plugins are able to execute code during a Composer run.
    ///
    /// When a new plugin is first activated, which is not yet listed in the config option,
    /// Composer will print a warning.
    /// If you run Composer interactively it will prompt you to decide if you want to execute the plugin or not.
    ///
    /// Use this setting to allow only packages you trust to execute code.
    /// Set it to an object with package name patterns as keys.
    /// The values are `true` to allow and `false` to disallow while suppressing
    /// further warnings and prompts.
    ///
    /// ```json
    /// {
    ///     "config": {
    ///         "allow-plugins": {
    ///             "third-party/required-plugin": true,
    ///             "my-organization/*": true,
    ///             "unnecessary/plugin": false
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// You can also set the config option itself to `false` to disallow all plugins,
    /// or `true` to allow all plugins to run (NOT recommended).
    /// For example:
    ///
    /// ```json
    /// {
    ///     "config": {
    ///         "allow-plugins": false
    ///     }
    /// }
    /// ```
    ///
    /// Reference: [Config (allow-plugins)](https://getcomposer.org/doc/06-config.md#allow-plugins).
    #[serde(rename = "allow-plugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_plugins: Option<AllowPlugins>,

    /// Defaults to `false`.
    /// If `true`, the Composer autoloader will also look for classes in the PHP include path.
    ///
    /// Reference: [Config (use-include-path)](https://getcomposer.org/doc/06-config.md#use-include-path).
    #[serde(rename = "use-include-path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    use_include_path: Option<bool>,

    /// Defaults to `dist` and can be any of `source`, `dist` or `auto`.
    /// This option allows you to set the install method Composer will prefer to use.
    /// Can optionally be an object with package name patterns for keys
    /// for more granular install preferences.
    ///
    /// ```json
    /// {
    ///     "config": {
    ///         "preferred-install": {
    ///             "my-organization/stable-package": "dist",
    ///             "my-organization/*": "source",
    ///             "partner-organization/*": "auto",
    ///             "*": "dist"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// - `source` means Composer will install packages from their `source` if there is one.
    ///   This is typically a git clone or equivalent checkout of the version control system the package uses.
    ///   This is useful if you want to make a bugfix to a project and get a local git clone of the dependency directly.
    /// - `auto` is the legacy behavior where Composer uses `source` automatically for dev versions, and `dist` otherwise.
    /// - `dist` (the default as of Composer 2.1) means Composer installs from `dist`, where possible.
    ///   This is typically a zip file download, which is faster than cloning the entire repository.
    ///
    /// **Note:**
    /// Order matters. More specific patterns should be earlier than more relaxed patterns.
    /// When mixing the string notation with the hash configuration in global and package
    /// configurations the string notation is translated to a `*` package pattern.
    ///
    /// Reference: [Config (preferred-install)](https://getcomposer.org/doc/06-config.md#preferred-install).
    #[serde(rename = "preferred-install")]
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_install: Option<PreferredInstall>,

    /// Security audit configuration options.
    ///
    /// Reference: [Config (audit)](https://getcomposer.org/doc/06-config.md#audit).
    #[serde(rename = "audit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    audit: Option<Audit>,

    /// When running Composer in a directory where there is no composer.json, if there is one
    /// present in a directory above Composer will by default ask you whether you want to use
    /// that directory's composer.json instead.
    ///
    /// If you always want to answer yes to this prompt, you can set this config value to `true`.
    /// To never be prompted, set it to `false`. The default is `"prompt"`.
    ///
    /// **Note:**
    /// This config must be set in your global user-wide config for it to work.
    /// Use for example `php composer.phar config --global use-parent-dir true` to set it.
    ///
    /// Reference: [Config (use-parent-dir)](https://getcomposer.org/doc/06-config.md#use-parent-dir).
    #[serde(rename = "use-parent-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    use_parent_dir: Option<bool>,

    /// What to do after prompting for authentication, one of:
    /// - `true` (always store),
    /// - `false` (do not store), and
    /// - `"prompt"` (ask every time)
    ///
    /// Defaults to `"prompt"`.
    ///
    /// Reference: [Config (store-auths)](https://getcomposer.org/doc/06-config.md#store-auths).
    #[serde(rename = "store-auths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    store_auths: Option<ConfigStoreAuths>,

    /// A list of protocols to use when cloning from github.com, in priority order.
    /// By default `git` is present but only if [secure-http](https://getcomposer.org/doc/06-config.md#secure-http)
    /// is disabled, as the git protocol is not encrypted.
    /// If you want your origin remote push URLs to be using https and not
    /// ssh (`git@github.com:...`), then set the protocol list to be only
    /// ["https"] and Composer will stop overwriting the push URL to an ssh URL.
    ///
    /// Defaults to ["https", "ssh", "git"].
    ///
    /// Reference: [Config (github-protocols)](https://getcomposer.org/doc/06-config.md#github-protocols).
    #[serde(rename = "github-protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    github_protocols: Option<Vec<String>>,

    /// A list of domain names and oauth keys.
    /// For example using `{"github.com": "oauthtoken"}` as the value of this
    /// option will use `oauthtoken` to access private repositories on github
    /// and to circumvent the low IP-based rate limiting of their API.
    /// Composer may prompt for credentials when needed, but these can also be manually set.
    /// Read more on how to get an OAuth token for GitHub and cli syntax [here](https://getcomposer.org/doc/articles/authentication-for-private-packages.md#github-oauth).
    ///
    /// Reference: [Config (github-oauth)](https://getcomposer.org/doc/06-config.md#github-oauth).
    #[serde(rename = "github-oauth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    github_oauth: Option<HashMap<String, String>>,

    /// Defaults to ["gitlab.com"].
    /// A list of domains of GitLab servers.
    /// This is used if you use the `gitlab` repository type.
    ///
    /// Reference: [Config (gitlab-domains)](https://getcomposer.org/doc/06-config.md#gitlab-domains).
    #[serde(rename = "gitlab-domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_domains: Option<Vec<String>>,

    /// A list of domain names and oauth keys.
    ///
    /// For example using `{"gitlab.com": "oauthtoken"}` as the value of this option
    /// will use `oauthtoken` to access private repositories on gitlab.
    ///
    /// **Please note:** If the package is not hosted at `gitlab.com` the domain
    /// names must be also specified with the `gitlab-domains` option.
    ///
    /// Further info can also be found [here](https://getcomposer.org/doc/articles/authentication-for-private-packages.md#gitlab-oauth).
    ///
    /// Reference: [Config (gitlab-oauth)](https://getcomposer.org/doc/06-config.md#gitlab-oauth).
    #[serde(rename = "gitlab-oauth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_oauth: Option<HashMap<String, String>>,

    /// A list of domain names and private tokens.
    /// Private token can be either simple string, or array with username and token.
    /// For example using `{"gitlab.com": "privatetoken"}` as the value of this option
    /// will use `privatetoken` to access private repositories on gitlab.
    /// Using `{"gitlab.com": {"username": "gitlabuser", "token": "privatetoken"}}`
    /// will use both username and token for gitlab deploy token functionality (https://docs.gitlab.com/ee/user/project/deploy_tokens/).
    ///
    /// **Please note:** If the package is not hosted at gitlab.com the domain
    /// names must be also specified with the `gitlab-domains` option.
    /// The token must have `api` or `read_api` scope.
    /// Further info can also be found [here](https://getcomposer.org/doc/articles/authentication-for-private-packages.md#gitlab-token).
    ///
    /// Reference: [Config (gitlab-token)](https://getcomposer.org/doc/06-config.md#gitlab-token).
    #[serde(rename = "gitlab-token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_token: Option<GitlabTokenConfig>,

    /// A protocol to force use of when creating a repository URL for the `source` value
    /// of the package metadata.
    /// One of `git` or `http` (`https` is treated as a synonym for `http`).
    /// Helpful when working with projects referencing private repositories which will
    /// later be cloned in GitLab CI jobs with a [GitLab `CI_JOB_TOKEN`](https://docs.gitlab.com/ee/ci/variables/predefined_variables.html#predefined-variables-reference)
    /// using HTTP basic auth.
    /// By default, Composer will generate a git-over-SSH URL for private repositories and HTTP(S) only for public.
    ///
    /// Reference: [Config (gitlab-protocol)](https://getcomposer.org/doc/06-config.md#gitlab-protocol).
    #[serde(rename = "gitlab-protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_protocol: Option<GitProtocol>,

    /// Defaults to `false`.
    /// If set to `true` all HTTPS URLs will be tried with HTTP instead and no
    /// network level encryption is performed.
    /// Enabling this is a security risk and is NOT recommended.
    /// The better way is to enable the php_openssl extension in php.ini.
    /// Enabling this will implicitly disable the `secure-http` option.
    ///
    /// Reference: [Config (disable-tls)](https://getcomposer.org/doc/06-config.md#disable-tls).
    #[serde(rename = "disable-tls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_tls: Option<bool>,

    /// Defaults to `true`.
    /// If set to `true` only HTTPS URLs are allowed to be downloaded via Composer.
    /// If you really absolutely need HTTP access to something then you can disable it,
    /// but using [Let's Encrypt](https://letsencrypt.org/) to get a free SSL certificate is generally a better alternative.
    ///
    /// Reference: [Config (secure-http)](https://getcomposer.org/doc/06-config.md#secure-http).
    #[serde(rename = "secure-http")]
    #[serde(skip_serializing_if = "Option::is_none")]
    secure_http: Option<bool>,

    /// A list of domain names and consumers.
    ///
    /// **Example:**
    /// ```json
    /// {
    ///     "bitbucket.org": {
    ///         "consumer-key": "myKey",
    ///         "consumer-secret": "mySecret"
    ///     }
    /// }
    /// ```
    ///
    /// Read more [here](https://getcomposer.org/doc/articles/authentication-for-private-packages.md#bitbucket-oauth).
    ///
    /// Reference: [Config (bitbucket-oauth)](https://getcomposer.org/doc/06-config.md#bitbucket-oauth).
    #[serde(rename = "bitbucket-oauth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_oauth: Option<HashMap<BitbucketHost, BitbucketOauth>>,

    /// Location of Certificate Authority file on local filesystem.
    /// In PHP 5.6+ you should rather set this via openssl.cafile in php.ini,
    /// although PHP 5.6+ should be able to detect your system CA file automatically.
    ///
    /// Reference: [Config ("cafile")](https://getcomposer.org/doc/06-config.md#"cafile").
    #[serde(rename = "cafile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cafile: Option<String>,

    /// If cafile is not specified or if the certificate is not found there,
    /// the directory pointed to by capath is searched for a suitable certificate.
    /// `capath` must be a correctly hashed certificate directory.
    ///
    /// Reference: [Config ("capath")](https://getcomposer.org/doc/06-config.md#"capath").
    #[serde(rename = "capath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    capath: Option<String>,

    /// A list of domain names and username/passwords to authenticate against them.
    ///
    /// For example using
    /// ```json
    /// {
    ///     "example.org": {
    ///         "username": "alice",
    ///         "password": "foo"
    ///     }
    /// }
    /// ```
    /// as the value of this option will let Composer authenticate against example.org.
    ///
    /// More info can be found [here](https://getcomposer.org/doc/articles/authentication-for-private-packages.md#http-basic).
    ///
    /// Reference: [Config (http-basic)](https://getcomposer.org/doc/06-config.md#http-basic).
    #[serde(rename = "http-basic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    http_basic: Option<HttpBasicAuth>,

    /// A list of domain names and tokens to authenticate against them.
    ///
    /// For example using
    /// ```json
    /// {
    ///     "example.org": "foo"
    /// }
    /// ```
    /// as the value of this option will let Composer authenticate
    /// against example.org using an `Authorization: Bearer foo` header.
    ///
    /// Reference: [Config ("bearer")](https://getcomposer.org/doc/06-config.md#"bearer").
    #[serde(rename = "bearer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    bearer: Option<HashMap<Host, String>>,

    /// Lets you fake platform packages (PHP and extensions) so that you can emulate
    /// a production env or define your target platform in the config.
    ///
    /// **Example:**
    /// ```json
    /// {
    ///     "php": "7.0.3",
    ///     "ext-something": "4.0.3"
    /// }
    /// ```
    ///
    /// This will make sure that no package requiring more than PHP 7.0.3 can be installed regardless
    /// of the actual PHP version you run locally. However it also means the dependencies are not
    /// checked correctly anymore, if you run PHP 5.6 it will install fine as it assumes 7.0.3,
    /// but then it will fail at runtime.
    /// This also means if `{"php":"7.4"}` is specified; no packages will be used
    /// that define `7.4.1` as minimum.
    ///
    /// Therefore if you use this it is recommended, and safer, to also run the
    /// [check-platform-reqs](https://getcomposer.org/doc/03-cli.md#check-platform-reqs)
    /// command as part of your deployment strategy.
    ///
    /// If a dependency requires some extension that you do not have installed locally you may
    /// ignore it instead by passing `--ignore-platform-req=ext-foo` to `update`, `install` or `require`.
    /// In the long run though you should install required extensions as if you ignore one now and a
    /// new package you add a month later also requires it, you may introduce issues in production unknowingly.
    ///
    /// If you have an extension installed locally but not on production, you may want to artificially
    /// hide it from Composer using `{"ext-foo": false}`.
    ///
    /// Reference: [Config ("platform")](https://getcomposer.org/doc/06-config.md#"platform").
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<HashMap<PlatformPackage, PlatformConstraint>>,

    /// Defaults to `vendor`.
    /// You can install dependencies into a different directory if you want to.
    /// `$HOME` and `~` will be replaced by your home directory's path in `vendor-dir`
    /// and all `*-dir` options below.
    ///
    /// Reference: [Config (vendor-dir)](https://getcomposer.org/doc/06-config.md#vendor-dir).
    #[serde(rename = "vendor-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor_dir: Option<String>,

    /// Defaults to `vendor/bin`.
    /// If a project includes binaries, they will be symlinked into this directory.
    ///
    /// Reference: [Config (bin-dir)](https://getcomposer.org/doc/06-config.md#bin-dir).
    #[serde(rename = "bin-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    bin_dir: Option<String>,

    /// Defaults to `C:\Users\<user>\AppData\Roaming\Composer` on Windows,
    /// `$XDG_DATA_HOME/composer` on unix systems that follow the XDG Base Directory Specifications,
    /// and `$COMPOSER_HOME` on other unix systems.
    /// Right now it is only used for storing past composer.phar files to be able to roll back to older versions.
    /// See also [COMPOSER_HOME](https://getcomposer.org/doc/03-cli.md#composer-home).
    ///
    /// Reference: [Config (data-dir)](https://getcomposer.org/doc/06-config.md#data-dir).
    #[serde(rename = "data-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_dir: Option<String>,

    /// Defaults to `C:\Users\<user>\AppData\Local\Composer` on Windows,
    /// `/Users/<user>/Library/Caches/composer` on macOS,
    /// `$XDG_CACHE_HOME/composer` on unix systems that follow the XDG Base
    /// Directory Specifications, and `$COMPOSER_HOME/cache` on other unix systems.
    /// Stores all the caches used by Composer.
    /// See also [COMPOSER_HOME](https://getcomposer.org/doc/03-cli.md#composer-home).
    ///
    /// Reference: [Config (cache-dir)](https://getcomposer.org/doc/06-config.md#cache-dir).
    #[serde(rename = "cache-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_dir: Option<String>,

    /// Defaults to `$cache-dir/files`.
    /// Stores the zip archives of packages.
    ///
    /// Reference: [Config (cache-files-dir)](https://getcomposer.org/doc/06-config.md#cache-files-dir).
    #[serde(rename = "cache-files-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_files_dir: Option<String>,

    /// Defaults to `$cache-dir/repo`.
    /// Stores repository metadata for the `composer` type and the VCS repos of
    /// type `svn`, `fossil`, `github` and `bitbucket`.
    ///
    /// Reference: [Config (cache-repo-dir)](https://getcomposer.org/doc/06-config.md#cache-repo-dir).
    #[serde(rename = "cache-repo-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_repo_dir: Option<String>,

    /// Defaults to `$cache-dir/vcs`.
    /// Stores VCS clones for loading VCS repository metadata for the `git`/`hg`
    /// types and to speed up installs.
    ///
    /// Reference: [Config (cache-vcs-dir)](https://getcomposer.org/doc/06-config.md#cache-vcs-dir).
    #[serde(rename = "cache-vcs-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_vcs_dir: Option<String>,

    /// Defaults to `15552000` (6 months).
    /// Composer caches all dist (zip, tar, ...) packages that it downloads.
    /// Those are purged after six months of being unused by default.
    /// This option allows you to tweak this duration (in seconds) or disable it completely by setting it to 0.
    ///
    /// Reference: [Config (cache-files-ttl)](https://getcomposer.org/doc/06-config.md#cache-files-ttl).
    #[serde(rename = "cache-files-ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_files_ttl: Option<u32>,

    /// Defaults to `300MiB`. Composer caches all dist (zip, tar, ...) packages that it downloads.
    /// When the garbage collection is periodically ran, this is the maximum size the cache will
    /// be able to use.
    /// Older (less used) files will be removed first until the cache fits.
    ///
    /// Reference: [Config (cache-files-maxsize)](https://getcomposer.org/doc/06-config.md#cache-files-maxsize).
    #[serde(rename = "cache-files-maxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_files_maxsize: Option<String>,

    /// Defaults to `false`.
    /// Whether to use the Composer cache in read-only mode.
    ///
    /// Reference: [Config (cache-read-only)](https://getcomposer.org/doc/06-config.md#cache-read-only).
    #[serde(rename = "cache-read-only")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_read_only: Option<bool>,

    /// Defaults to `auto`.
    /// Determines the compatibility of the binaries to be installed. If it is
    /// `auto` then Composer only installs .bat proxy files when on Windows or WSL.
    /// If set to `full` then both .bat files for Windows and scripts for Unix-based
    /// operating systems will be installed for each binary.
    /// This is mainly useful if you run Composer inside a linux VM but still want
    /// the .bat proxies available for use in the Windows host OS.
    /// If set to `proxy` Composer will only create bash/Unix-style proxy files and
    /// no .bat files even on Windows/WSL.
    ///
    /// Reference: [Config (bin-compat)](https://getcomposer.org/doc/06-config.md#bin-compat).
    #[serde(rename = "bin-compat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    bin_compat: Option<BinaryCompatibility>,

    /// Defaults to `true`.
    /// If `false`, the Composer autoloader will not be prepended to existing autoloaders.
    /// This is sometimes required to fix interoperability issues with other autoloaders.
    ///
    /// Reference: [Config (prepend-autoloader)](https://getcomposer.org/doc/06-config.md#prepend-autoloader).
    #[serde(rename = "prepend-autoloader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    prepend_autoloader: Option<bool>,

    /// Defaults to `null`.
    /// When set to a non-empty string, this value will be used as a suffix for the
    /// generated Composer autoloader.
    /// If set to `null`, the `content-hash` value from the `composer.lock` file will
    /// be used if available; otherwise, a random suffix will be generated.
    ///
    /// Reference: [Config (autoloader-suffix)](https://getcomposer.org/doc/06-config.md#autoloader-suffix).
    #[serde(rename = "autoloader-suffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    autoloader_suffix: Option<String>,

    /// Defaults to `false`.
    /// If `true`, always optimize when dumping the autoloader.
    ///
    /// Reference: [Config (optimize-autoloader)](https://getcomposer.org/doc/06-config.md#optimize-autoloader).
    #[serde(rename = "optimize-autoloader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    optimize_autoloader: Option<bool>,

    /// Defaults to `false`.
    /// If `true`, the require command keeps packages sorted by name in
    /// `composer.json` when adding a new package.
    ///
    /// Reference: [Config (sort-packages)](https://getcomposer.org/doc/06-config.md#sort-packages).
    #[serde(rename = "sort-packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_packages: Option<bool>,

    /// Defaults to `false`.
    /// If `true`, the Composer autoloader will only load classes from the classmap.
    /// Implies `optimize-autoloader`.
    ///
    /// Reference: [Config (classmap-authoritative)](https://getcomposer.org/doc/06-config.md#classmap-authoritative).
    #[serde(rename = "classmap-authoritative")]
    #[serde(skip_serializing_if = "Option::is_none")]
    classmap_authoritative: Option<bool>,

    /// Defaults to `false`.
    /// If `true`, the Composer autoloader will check for APCu and use it to
    /// cache found/not-found classes when the extension is enabled.
    ///
    /// Reference: [Config (apcu-autoloader)](https://getcomposer.org/doc/06-config.md#apcu-autoloader).
    #[serde(rename = "apcu-autoloader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    apcu_autoloader: Option<bool>,

    /// Defaults to `["github.com"]`.
    /// A list of domains to use in github mode.
    /// This is used for GitHub Enterprise setups.
    ///
    /// Reference: [Config (github-domains)](https://getcomposer.org/doc/06-config.md#github-domains).
    #[serde(rename = "github-domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    github_domains: Option<Vec<GithubHost>>,

    /// Defaults to `true`.
    /// If `false`, the OAuth tokens created to access the github API will have
    /// a date instead of the machine hostname.
    ///
    /// Reference: [Config (github-expose-hostname)](https://getcomposer.org/doc/06-config.md#github-expose-hostname).
    #[serde(rename = "github-expose-hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    github_expose_hostname: Option<bool>,

    /// Defaults to `true`.
    /// Similar to the `no-api` key on a specific repository, setting `use-github-api`
    /// to `false` will define the global behavior for all GitHub repositories to
    /// clone the repository as it would with any other git repository instead of
    /// using the GitHub API.
    /// But unlike using the `git` driver directly, Composer will still attempt
    /// to use GitHub's zip files.
    ///
    /// Reference: [Config (use-github-api)](https://getcomposer.org/doc/06-config.md#use-github-api).
    #[serde(rename = "use-github-api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    use_github_api: Option<bool>,

    /// Defaults to `true`.
    /// Composer allows repositories to define a notification URL, so that they
    /// get notified whenever a package from that repository is installed.
    /// This option allows you to disable that behavior.
    ///
    /// Reference: [Config (notify-on-install)](https://getcomposer.org/doc/06-config.md#notify-on-install).
    #[serde(rename = "notify-on-install")]
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_on_install: Option<bool>,

    /// Defaults to `false` and can be any of `true`, `false` or `"stash"`.
    /// This option allows you to set the default style of handling dirty updates
    /// when in non-interactive mode.
    /// `true` will always discard changes in vendors, while `"stash"` will try
    /// to stash and reapply.
    /// Use this for CI servers or deploy scripts if you tend to have modified vendors.
    ///
    /// Reference: [Config (discard-changes)](https://getcomposer.org/doc/06-config.md#discard-changes).
    #[serde(rename = "discard-changes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    discard_changes: Option<DiscardChangesMode>,

    /// Defaults to `tar`.
    /// Overrides the default format used by the archive command.
    ///
    /// Reference: [Config (archive-format)](https://getcomposer.org/doc/06-config.md#archive-format).
    #[serde(rename = "archive-format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_format: Option<ArchiveFormat>,

    /// Defaults to `.`.
    /// Default destination for archives created by the archive command.
    ///
    /// **Example:**
    /// ```json
    /// {
    ///     "config": {
    ///         "archive-dir": "/home/user/.composer/repo"
    ///     }
    /// }
    /// ```
    ///
    /// Reference: [Config (archive-dir)](https://getcomposer.org/doc/06-config.md#archive-dir).
    #[serde(rename = "archive-dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_dir: Option<String>,

    /// Defaults to `true`.
    /// If set to `false`, Composer will not create `.htaccess` files in the
    /// Composer home, cache, and data directories.
    ///
    /// Reference: [Config (htaccess-protect)](https://getcomposer.org/doc/06-config.md#htaccess-protect).
    #[serde(rename = "htaccess-protect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    htaccess_protect: Option<bool>,

    /// Defaults to `true`.
    /// If set to `false`, Composer will not create a `composer.lock` file and
    /// will ignore it if one is present.
    ///
    /// Reference: [Config (lock)](https://getcomposer.org/doc/06-config.md#lock).
    #[serde(rename = "lock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    lock: Option<bool>,

    /// Defaults to `php-only` which only checks the PHP version.
    /// Set to `true` to also check the presence of extension.
    /// If set to `false`, Composer will not create and require a `platform_check.php`
    /// file as part of the autoloader bootstrap.
    ///
    /// Reference: [Config (platform-check)](https://getcomposer.org/doc/06-config.md#platform-check).
    #[serde(rename = "platform-check")]
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_check: Option<PlatformCheckMode>,

    /// Defaults to `[]`.
    /// Lists domains which should be trusted/marked as using a secure Subversion/SVN transport.
    /// By default svn:// protocol is seen as insecure and will throw, but you
    /// can set this config option to `["example.org"]` to allow using svn URLs
    /// on that hostname.
    /// This is a better/safer alternative to disabling `secure-http` altogether.
    ///
    /// Reference: [Config (secure-svn-domains)](https://getcomposer.org/doc/06-config.md#secure-svn-domains).
    #[serde(rename = "secure-svn-domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    secure_svn_domains: Option<Vec<Host>>,
}

/// What to do after prompting for authentication, one of:
/// - `true` (always store),
/// - `false` (do not store), and
/// - `"prompt"` (ask every time)
#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigStoreAuths {
    #[serde(rename = "true")]
    AlwaysStore,

    #[serde(rename = "false")]
    DoNotStore,

    #[serde(rename = "prompt")]
    AskEveryTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AllowPlugins {
    Toggle(bool),
    Map(HashMap<String, bool>),
}

/// Security audit configuration options.
///
/// Reference [Config ()](https://getcomposer.org/doc/06-config.md#).
#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    /// A set of advisory ids, remote ids or CVE ids that should be ignored and not reported as part of an audit.
    ///
    /// ```json
    /// {
    ///     "config": {
    ///         "audit": {
    ///             "ignored": ["CVE-1234", "GHSA-xx", "PKSA-yy"]
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Reference [Config (ignored)](https://getcomposer.org/doc/06-config.md#ignored).
    ignored: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScriptEventType {
    Command(CommandEvent),
    Installer(InstallerEvent),
    Package(PackageEvent),
    Plugin(PluginEvent),
}

/// Reference: [Scripts (Command Events)](https://getcomposer.org/doc/articles/scripts.md#command-events).
#[derive(Debug, Serialize, Deserialize)]
pub enum CommandEvent {
    /// Occurs before the `install` command is executed with a lock file present.
    #[serde(rename = "pre-install-cmd")]
    PreInstallCmd,

    /// Occurs after the `install` command has been executed with a lock file present.
    #[serde(rename = "post-install-cmd")]
    PostInstallCmd,

    /// Occurs before the `update` command is executed, or before the install command is executed without a lock file present.
    #[serde(rename = "pre-update-cmd")]
    PreUpdateCmd,

    /// Occurs after the `update` command has been executed, or after the install command has been executed without a lock file present.
    #[serde(rename = "post-update-cmd")]
    PostUpdateCmd,

    /// Occurs before the `status` command is executed.
    #[serde(rename = "pre-status-cmd")]
    PreStatusCmd,

    /// Occurs after the `status` command has been executed.
    #[serde(rename = "post-status-cmd")]
    PostStatusCmd,

    /// Occurs before the `archive` command is executed.
    #[serde(rename = "pre-archive-cmd")]
    PreArchiveCmd,

    /// Occurs after the `archive` command has been executed.
    #[serde(rename = "post-archive-cmd")]
    PostArchiveCmd,

    /// Occurs before the autoloader is dumped, either during `install`/`update`, or via the `dump-autoload` command.
    #[serde(rename = "pre-autoload-dump")]
    PreAutoloadDump,

    /// Occurs after the autoloader has been dumped, either during `install`/`update`, or via the `dump-autoload` command.
    #[serde(rename = "post-autoload-dump")]
    PostAutoloadDump,

    /// Occurs after the root package has been installed during the `create-project` command (but before its dependencies are installed).
    #[serde(rename = "post-root-package-install")]
    PostRootPackageInstall,

    /// Occurs after the `create-project` command has been executed.
    #[serde(rename = "post-create-project-cmd")]
    PostCreateProjectCmd,
}

/// Reference: [Scripts (Installer Events)](https://getcomposer.org/doc/articles/scripts.md#installer-events).
#[derive(Debug, Serialize, Deserialize)]
pub enum InstallerEvent {
    /// Occurs before the install/upgrade/.. operations are executed when installing a lock file.
    /// Plugins that need to hook into this event will need to be installed globally to be usable,
    /// as otherwise they would not be loaded yet when a fresh install of a project happens.
    #[serde(rename = "pre-operations-exec")]
    PreOperationsExec,
}

/// Reference: [Scripts (Package Events)](https://getcomposer.org/doc/articles/scripts.md#package-events).
#[derive(Debug, Serialize, Deserialize)]
pub enum PackageEvent {
    /// Occurs before a package is installed.
    #[serde(rename = "pre-package-install")]
    PrePackageInstall,

    /// Occurs after a package has been installed.
    #[serde(rename = "post-package-install")]
    PostPackageInstall,

    /// Occurs before a package is updated.
    #[serde(rename = "pre-package-update")]
    PrePackageUpdate,

    /// Occurs after a package has been updated.
    #[serde(rename = "post-package-update")]
    PostPackageUpdate,

    /// Occurs before a package is uninstalled.
    #[serde(rename = "pre-package-uninstall")]
    PrePackageUninstall,

    /// Occurs after a package has been uninstalled.
    #[serde(rename = "post-package-uninstall")]
    PostPackageUninstall,
}

/// Reference: [Scripts (Plugin Events)](https://getcomposer.org/doc/articles/scripts.md#plugin-events).
#[derive(Debug, Serialize, Deserialize)]
pub enum PluginEvent {
    /// Occurs after a Composer instance is done being initialized.
    #[serde(rename = "init")]
    Init,

    /// Occurs before any Composer Command is executed on the CLI.
    /// It provides you with access to the input and output objects of the program.
    #[serde(rename = "command")]
    Command,

    /// Occurs before files are downloaded and allows you to manipulate the `HttpDownloader`
    /// object prior to downloading files based on the URL to be downloaded.
    #[serde(rename = "pre-file-download")]
    PreFileDownload,

    /// Occurs after package dist files are downloaded and allows you to perform
    /// additional checks on the file if required.
    #[serde(rename = "post-file-download")]
    PostFileDownload,

    /// Occurs before a command is executed and allows you to manipulate the `InputInterface`
    /// object's options and arguments to tweak a command's behavior.
    #[serde(rename = "pre-command-run")]
    PreCommandRun,

    /// Occurs before the Pool of packages is created, and lets you filter the
    /// list of packages that is going to enter the Solver.
    #[serde(rename = "pre-pool-create")]
    PrePoolCreate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scripts {
    /// Occurs before the `install` command is executed with a lock file present.
    #[serde(rename = "pre-install-cmd")]
    pre_install_cmd: OneOrMany<String>,

    /// Occurs after the `install` command has been executed with a lock file present.
    #[serde(rename = "post-install-cmd")]
    post_install_cmd: OneOrMany<String>,

    /// Occurs before the `update` command is executed, or before the install command is executed without a lock file present.
    #[serde(rename = "pre-update-cmd")]
    pre_update_cmd: OneOrMany<String>,

    /// Occurs after the `update` command has been executed, or after the install command has been executed without a lock file present.
    #[serde(rename = "post-update-cmd")]
    post_update_cmd: OneOrMany<String>,

    /// Occurs before the `status` command is executed.
    #[serde(rename = "pre-status-cmd")]
    pre_status_cmd: OneOrMany<String>,

    /// Occurs after the `status` command has been executed.
    #[serde(rename = "post-status-cmd")]
    post_status_cmd: OneOrMany<String>,

    /// Occurs before the `archive` command is executed.
    #[serde(rename = "pre-archive-cmd")]
    pre_archive_cmd: OneOrMany<String>,

    /// Occurs after the `archive` command has been executed.
    #[serde(rename = "post-archive-cmd")]
    post_archive_cmd: OneOrMany<String>,

    /// Occurs before the autoloader is dumped, either during `install`/`update`, or via the `dump-autoload` command.
    #[serde(rename = "pre-autoload-dump")]
    pre_autoload_dump: OneOrMany<String>,

    /// Occurs after the autoloader has been dumped, either during `install`/`update`, or via the `dump-autoload` command.
    #[serde(rename = "post-autoload-dump")]
    post_autoload_dump: OneOrMany<String>,

    /// Occurs after the root package has been installed during the `create-project` command (but before its dependencies are installed).
    #[serde(rename = "post-root-package-install")]
    post_root_package_install: OneOrMany<String>,

    /// Occurs after the `create-project` command has been executed.
    #[serde(rename = "post-create-project-cmd")]
    post_create_project_cmd: OneOrMany<String>,

    /// Occurs before the install/upgrade/.. operations are executed when installing a lock file.
    /// Plugins that need to hook into this event will need to be installed globally to be usable,
    /// as otherwise they would not be loaded yet when a fresh install of a project happens.
    #[serde(rename = "pre-operations-exec")]
    pre_operations_exec: OneOrMany<String>,

    /// Occurs before a package is installed.
    #[serde(rename = "pre-package-install")]
    pre_package_install: OneOrMany<String>,

    /// Occurs after a package has been installed.
    #[serde(rename = "post-package-install")]
    post_package_install: OneOrMany<String>,

    /// Occurs before a package is updated.
    #[serde(rename = "pre-package-update")]
    pre_package_update: OneOrMany<String>,

    /// Occurs after a package has been updated.
    #[serde(rename = "post-package-update")]
    post_package_update: OneOrMany<String>,

    /// Occurs before a package is uninstalled.
    #[serde(rename = "pre-package-uninstall")]
    pre_package_uninstall: OneOrMany<String>,

    /// Occurs after a package has been uninstalled.
    #[serde(rename = "post-package-uninstall")]
    post_package_uninstall: OneOrMany<String>,

    /// Occurs after a Composer instance is done being initialized.
    #[serde(rename = "init")]
    init: OneOrMany<String>,

    /// Occurs before any Composer Command is executed on the CLI.
    /// It provides you with access to the input and output objects of the program.
    #[serde(rename = "command")]
    command: OneOrMany<String>,

    /// Occurs before files are downloaded and allows you to manipulate the `HttpDownloader`
    /// object prior to downloading files based on the URL to be downloaded.
    #[serde(rename = "pre-file-download")]
    pre_file_download: OneOrMany<String>,

    /// Occurs after package dist files are downloaded and allows you to perform
    /// additional checks on the file if required.
    #[serde(rename = "post-file-download")]
    post_file_download: OneOrMany<String>,

    /// Occurs before a command is executed and allows you to manipulate the `InputInterface`
    /// object's options and arguments to tweak a command's behavior.
    #[serde(rename = "pre-command-run")]
    pre_command_run: OneOrMany<String>,

    /// Occurs before the Pool of packages is created, and lets you filter the
    /// list of packages that is going to enter the Solver.
    #[serde(rename = "pre-pool-create")]
    pre_pool_create: OneOrMany<String>,
}

/// A set of options for creating package archives.
///
/// The following options are supported:
///
/// - **name**: Allows configuring base name for archive.
///             By default (if not configured, and `--file` is not passed as command-line argument),
///             `preg_replace('#[^a-z0-9-_]#i', '-', name)` is used.
///
/// **Example:**
///
/// ```json
/// {
///     "name": "org/strangeName",
///     "archive": {
///         "name": "Strange_name"
///     }
/// }
/// ```
///
/// - **exclude**: Allows configuring a list of patterns for excluded paths. The pattern syntax
///                matches .gitignore files. A leading exclamation mark (!) will result in any
///                matching files to be included even if a previous pattern excluded them.
///                A leading slash will only match at the beginning of the project relative path.
///                An asterisk will not expand to a directory separator.
///
/// **Example:**
///
/// ```json
/// {
///     "archive": {
///         "exclude": ["/foo/bar", "baz", "/*.test", "!/foo/bar/baz"]
///     }
/// }
/// ```
///
/// The example will include `/dir/foo/bar/file`, `/foo/bar/baz`, `/file.php`, `/foo/my.test` but
/// it will exclude `/foo/bar/any`, `/foo/baz`, and `/my.test`.
///
/// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#archive) for details.
#[derive(Debug, Serialize, Deserialize)]
pub struct Archive {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

/// Indicates whether this package has been abandoned.
///
/// It can be boolean or a package name/URL pointing to a recommended alternative.
///
/// **Examples:**
///
/// Use `"abandoned": true` to indicate this package is abandoned.
/// Use `"abandoned": "monolog/monolog"` to indicate this package is abandoned,
/// and that the recommended alternative is `monolog/monolog`.
///
/// Defaults to `false`.
///
/// See [The composer.json schema](https://getcomposer.org/doc/04-schema.md#abandoned) for details.
#[derive(Debug, Serialize, Deserialize)]
pub enum Abandoned {
    Toggle(bool),
    RecommendedAlternative(String),
}
