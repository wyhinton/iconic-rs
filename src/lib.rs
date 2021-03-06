//! This crate contains the auto-generated mapping of icon name for several popular icon fonts.
//! This crate does not provide the fonts or svg's themselves, but provides eponymous pub constants for  
//! (such as `Icon::NetworkWifi`) to the character codepoint `\u{e1ba}`)
//! in Googles Material Icon Font ([https://material.io/tools/icons/](https://material.io/tools/icons/)) -
//! useful if you want to use the material-icons font in user interfaces created
//! in Rust
//!
//! ## Example
//!
//! ```rust
// ! use material_icons::{Icon, icon_to_char};
// ! let icon_char = icon_to_char(Icon::Rotation3d);
// ! assert_eq!('\u{e84d}', icon_char);
//! ```
//!
//! ## License (please read - regarding embedded font)
/// Octicons Icons
#[allow(dead_code)]
pub struct Octicons;
#[allow(dead_code)]
impl Octicons {
/// ## [heart](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/heart.svg)

	pub const HEART: u32 = 9829;
/// ## [zap](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/zap.svg)

	pub const ZAP: u32 = 9889;
/// ## [light-bulb](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/light-bulb.svg)

	pub const LIGHTBULB: u32 = 61440;
/// ## [repo](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/repo.svg)

	pub const REPO: u32 = 61441;
/// ## [repo-forked](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/repo-forked.svg)

	pub const REPOFORKED: u32 = 61442;
/// ## [repo-push](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/repo-push.svg)

	pub const REPOPUSH: u32 = 61445;
/// ## [repo-pull](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/repo-pull.svg)

	pub const REPOPULL: u32 = 61446;
/// ## [book](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/book.svg)

	pub const BOOK: u32 = 61447;
/// ## [octoface](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/octoface.svg)

	pub const OCTOFACE: u32 = 61448;
/// ## [git-pull-request](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/git-pull-request.svg)

	pub const GITPULLREQUEST: u32 = 61449;
/// ## [mark-github](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/mark-github.svg)

	pub const MARKGITHUB: u32 = 61450;
/// ## [cloud-download](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/cloud-download.svg)

	pub const CLOUDDOWNLOAD: u32 = 61451;
/// ## [cloud-upload](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/cloud-upload.svg)

	pub const CLOUDUPLOAD: u32 = 61452;
/// ## [keyboard](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/keyboard.svg)

	pub const KEYBOARD: u32 = 61453;
/// ## [gist](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/gist.svg)

	pub const GIST: u32 = 61454;
/// ## [file-code](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-code.svg)

	pub const FILECODE: u32 = 61456;
/// ## [file-text](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-text.svg)

	pub const FILETEXT: u32 = 61457;
/// ## [file-media](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-media.svg)

	pub const FILEMEDIA: u32 = 61458;
/// ## [file-zip](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-zip.svg)

	pub const FILEZIP: u32 = 61459;
/// ## [file-pdf](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-pdf.svg)

	pub const FILEPDF: u32 = 61460;
/// ## [tag](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/tag.svg)

	pub const TAG: u32 = 61461;
/// ## [file-directory](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-directory.svg)

	pub const FILEDIRECTORY: u32 = 61462;
/// ## [file-submodule](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-submodule.svg)

	pub const FILESUBMODULE: u32 = 61463;
/// ## [person](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/person.svg)

	pub const PERSON: u32 = 61464;
/// ## [jersey](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/jersey.svg)

	pub const JERSEY: u32 = 61465;
/// ## [git-commit](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/git-commit.svg)

	pub const GITCOMMIT: u32 = 61471;
/// ## [git-branch](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/git-branch.svg)

	pub const GITBRANCH: u32 = 61472;
/// ## [git-merge](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/git-merge.svg)

	pub const GITMERGE: u32 = 61475;
/// ## [mirror](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/mirror.svg)

	pub const MIRROR: u32 = 61476;
/// ## [issue-opened](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/issue-opened.svg)

	pub const ISSUEOPENED: u32 = 61478;
/// ## [issue-reopened](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/issue-reopened.svg)

	pub const ISSUEREOPENED: u32 = 61479;
/// ## [issue-closed](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/issue-closed.svg)

	pub const ISSUECLOSED: u32 = 61480;
/// ## [star](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/star.svg)

	pub const STAR: u32 = 61482;
/// ## [comment](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/comment.svg)

	pub const COMMENT: u32 = 61483;
/// ## [question](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/question.svg)

	pub const QUESTION: u32 = 61484;
/// ## [alert](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/alert.svg)

	pub const ALERT: u32 = 61485;
/// ## [search](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/search.svg)

	pub const SEARCH: u32 = 61486;
/// ## [gear](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/gear.svg)

	pub const GEAR: u32 = 61487;
/// ## [radio-tower](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/radio-tower.svg)

	pub const RADIOTOWER: u32 = 61488;
/// ## [tools](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/tools.svg)

	pub const TOOLS: u32 = 61489;
/// ## [sign-out](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/sign-out.svg)

	pub const SIGNOUT: u32 = 61490;
/// ## [rocket](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/rocket.svg)

	pub const ROCKET: u32 = 61491;
/// ## [rss](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/rss.svg)

	pub const RSS: u32 = 61492;
/// ## [clippy](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/clippy.svg)

	pub const CLIPPY: u32 = 61493;
/// ## [sign-in](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/sign-in.svg)

	pub const SIGNIN: u32 = 61494;
/// ## [organization](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/organization.svg)

	pub const ORGANIZATION: u32 = 61495;
/// ## [device-mobile](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/device-mobile.svg)

	pub const DEVICEMOBILE: u32 = 61496;
/// ## [unfold](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/unfold.svg)

	pub const UNFOLD: u32 = 61497;
/// ## [check](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/check.svg)

	pub const CHECK: u32 = 61498;
/// ## [mail](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/mail.svg)

	pub const MAIL: u32 = 61499;
/// ## [mail-read](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/mail-read.svg)

	pub const MAILREAD: u32 = 61500;
/// ## [arrow-up](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/arrow-up.svg)

	pub const ARROWUP: u32 = 61501;
/// ## [arrow-right](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/arrow-right.svg)

	pub const ARROWRIGHT: u32 = 61502;
/// ## [arrow-down](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/arrow-down.svg)

	pub const ARROWDOWN: u32 = 61503;
/// ## [arrow-left](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/arrow-left.svg)

	pub const ARROWLEFT: u32 = 61504;
/// ## [pin](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/pin.svg)

	pub const PIN: u32 = 61505;
/// ## [gift](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/gift.svg)

	pub const GIFT: u32 = 61506;
/// ## [graph](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/graph.svg)

	pub const GRAPH: u32 = 61507;
/// ## [triangle-left](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/triangle-left.svg)

	pub const TRIANGLELEFT: u32 = 61508;
/// ## [credit-card](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/credit-card.svg)

	pub const CREDITCARD: u32 = 61509;
/// ## [clock](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/clock.svg)

	pub const CLOCK: u32 = 61510;
/// ## [ruby](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/ruby.svg)

	pub const RUBY: u32 = 61511;
/// ## [broadcast](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/broadcast.svg)

	pub const BROADCAST: u32 = 61512;
/// ## [key](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/key.svg)

	pub const KEY: u32 = 61513;
/// ## [repo-force-push](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/repo-force-push.svg)

	pub const REPOFORCEPUSH: u32 = 61514;
/// ## [repo-clone](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/repo-clone.svg)

	pub const REPOCLONE: u32 = 61516;
/// ## [diff](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/diff.svg)

	pub const DIFF: u32 = 61517;
/// ## [eye](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/eye.svg)

	pub const EYE: u32 = 61518;
/// ## [comment-discussion](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/comment-discussion.svg)

	pub const COMMENTDISCUSSION: u32 = 61519;
/// ## [mail-reply](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/mail-reply.svg)

	pub const MAILREPLY: u32 = 61521;
/// ## [primitive-dot](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/primitive-dot.svg)

	pub const PRIMITIVEDOT: u32 = 61522;
/// ## [primitive-square](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/primitive-square.svg)

	pub const PRIMITIVESQUARE: u32 = 61523;
/// ## [device-camera](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/device-camera.svg)

	pub const DEVICECAMERA: u32 = 61526;
/// ## [device-camera-video](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/device-camera-video.svg)

	pub const DEVICECAMERAVIDEO: u32 = 61527;
/// ## [pencil](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/pencil.svg)

	pub const PENCIL: u32 = 61528;
/// ## [info](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/info.svg)

	pub const INFO: u32 = 61529;
/// ## [triangle-right](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/triangle-right.svg)

	pub const TRIANGLERIGHT: u32 = 61530;
/// ## [triangle-down](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/triangle-down.svg)

	pub const TRIANGLEDOWN: u32 = 61531;
/// ## [link](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/link.svg)

	pub const LINK: u32 = 61532;
/// ## [plus](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/plus.svg)

	pub const PLUS: u32 = 61533;
/// ## [three-bars](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/three-bars.svg)

	pub const THREEBARS: u32 = 61534;
/// ## [code](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/code.svg)

	pub const CODE: u32 = 61535;
/// ## [location](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/location.svg)

	pub const LOCATION: u32 = 61536;
/// ## [list-unordered](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/list-unordered.svg)

	pub const LISTUNORDERED: u32 = 61537;
/// ## [list-ordered](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/list-ordered.svg)

	pub const LISTORDERED: u32 = 61538;
/// ## [quote](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/quote.svg)

	pub const QUOTE: u32 = 61539;
/// ## [versions](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/versions.svg)

	pub const VERSIONS: u32 = 61540;
/// ## [calendar](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/calendar.svg)

	pub const CALENDAR: u32 = 61544;
/// ## [lock](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/lock.svg)

	pub const LOCK: u32 = 61546;
/// ## [diff-added](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/diff-added.svg)

	pub const DIFFADDED: u32 = 61547;
/// ## [diff-removed](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/diff-removed.svg)

	pub const DIFFREMOVED: u32 = 61548;
/// ## [diff-modified](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/diff-modified.svg)

	pub const DIFFMODIFIED: u32 = 61549;
/// ## [diff-renamed](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/diff-renamed.svg)

	pub const DIFFRENAMED: u32 = 61550;
/// ## [horizontal-rule](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/horizontal-rule.svg)

	pub const HORIZONTALRULE: u32 = 61552;
/// ## [arrow-small-right](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/arrow-small-right.svg)

	pub const ARROWSMALLRIGHT: u32 = 61553;
/// ## [milestone](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/milestone.svg)

	pub const MILESTONE: u32 = 61557;
/// ## [checklist](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/checklist.svg)

	pub const CHECKLIST: u32 = 61558;
/// ## [megaphone](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/megaphone.svg)

	pub const MEGAPHONE: u32 = 61559;
/// ## [chevron-right](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/chevron-right.svg)

	pub const CHEVRONRIGHT: u32 = 61560;
/// ## [bookmark](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/bookmark.svg)

	pub const BOOKMARK: u32 = 61563;
/// ## [settings](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/settings.svg)

	pub const SETTINGS: u32 = 61564;
/// ## [dashboard](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/dashboard.svg)

	pub const DASHBOARD: u32 = 61565;
/// ## [history](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/history.svg)

	pub const HISTORY: u32 = 61566;
/// ## [link-external](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/link-external.svg)

	pub const LINKEXTERNAL: u32 = 61567;
/// ## [mute](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/mute.svg)

	pub const MUTE: u32 = 61568;
/// ## [x](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/x.svg)

	pub const X: u32 = 61569;
/// ## [circle-slash](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/circle-slash.svg)

	pub const CIRCLESLASH: u32 = 61572;
/// ## [pulse](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/pulse.svg)

	pub const PULSE: u32 = 61573;
/// ## [sync](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/sync.svg)

	pub const SYNC: u32 = 61575;
/// ## [telescope](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/telescope.svg)

	pub const TELESCOPE: u32 = 61576;
/// ## [gist-secret](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/gist-secret.svg)

	pub const GISTSECRET: u32 = 61580;
/// ## [home](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/home.svg)

	pub const HOME: u32 = 61581;
/// ## [stop](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/stop.svg)

	pub const STOP: u32 = 61583;
/// ## [bug](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/bug.svg)

	pub const BUG: u32 = 61585;
/// ## [logo-github](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/logo-github.svg)

	pub const LOGOGITHUB: u32 = 61586;
/// ## [file-binary](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-binary.svg)

	pub const FILEBINARY: u32 = 61588;
/// ## [database](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/database.svg)

	pub const DATABASE: u32 = 61590;
/// ## [server](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/server.svg)

	pub const SERVER: u32 = 61591;
/// ## [diff-ignored](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/diff-ignored.svg)

	pub const DIFFIGNORED: u32 = 61593;
/// ## [ellipsis](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/ellipsis.svg)

	pub const ELLIPSIS: u32 = 61594;
/// ## [no-newline](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/no-newline.svg)

	pub const NONEWLINE: u32 = 61596;
/// ## [hubot](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/hubot.svg)

	pub const HUBOT: u32 = 61597;
/// ## [arrow-small-up](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/arrow-small-up.svg)

	pub const ARROWSMALLUP: u32 = 61599;
/// ## [arrow-small-down](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/arrow-small-down.svg)

	pub const ARROWSMALLDOWN: u32 = 61600;
/// ## [arrow-small-left](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/arrow-small-left.svg)

	pub const ARROWSMALLLEFT: u32 = 61601;
/// ## [chevron-up](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/chevron-up.svg)

	pub const CHEVRONUP: u32 = 61602;
/// ## [chevron-down](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/chevron-down.svg)

	pub const CHEVRONDOWN: u32 = 61603;
/// ## [chevron-left](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/chevron-left.svg)

	pub const CHEVRONLEFT: u32 = 61604;
/// ## [triangle-up](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/triangle-up.svg)

	pub const TRIANGLEUP: u32 = 61610;
/// ## [git-compare](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/git-compare.svg)

	pub const GITCOMPARE: u32 = 61612;
/// ## [logo-gist](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/logo-gist.svg)

	pub const LOGOGIST: u32 = 61613;
/// ## [file-symlink-file](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-symlink-file.svg)

	pub const FILESYMLINKFILE: u32 = 61616;
/// ## [file-symlink-directory](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/file-symlink-directory.svg)

	pub const FILESYMLINKDIRECTORY: u32 = 61617;
/// ## [squirrel](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/squirrel.svg)

	pub const SQUIRREL: u32 = 61618;
/// ## [globe](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/globe.svg)

	pub const GLOBE: u32 = 61622;
/// ## [unmute](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/unmute.svg)

	pub const UNMUTE: u32 = 61626;
/// ## [mention](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/mention.svg)

	pub const MENTION: u32 = 61630;
/// ## [package](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/package.svg)

	pub const PACKAGE: u32 = 61636;
/// ## [browser](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/browser.svg)

	pub const BROWSER: u32 = 61637;
/// ## [terminal](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/terminal.svg)

	pub const TERMINAL: u32 = 61640;
/// ## [markdown](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/markdown.svg)

	pub const MARKDOWN: u32 = 61641;
/// ## [dash](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/dash.svg)

	pub const DASH: u32 = 61642;
/// ## [fold](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/fold.svg)

	pub const FOLD: u32 = 61644;
/// ## [inbox](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/inbox.svg)

	pub const INBOX: u32 = 61647;
/// ## [trashcan](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/trashcan.svg)

	pub const TRASHCAN: u32 = 61648;
/// ## [paintcan](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/paintcan.svg)

	pub const PAINTCAN: u32 = 61649;
/// ## [flame](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/flame.svg)

	pub const FLAME: u32 = 61650;
/// ## [briefcase](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/briefcase.svg)

	pub const BRIEFCASE: u32 = 61651;
/// ## [plug](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/plug.svg)

	pub const PLUG: u32 = 61652;
/// ## [circuit-board](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/circuit-board.svg)

	pub const CIRCUITBOARD: u32 = 61654;
/// ## [mortar-board](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/mortar-board.svg)

	pub const MORTARBOARD: u32 = 61655;
/// ## [law](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/law.svg)

	pub const LAW: u32 = 61656;
/// ## [thumbsup](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/thumbsup.svg)

	pub const THUMBSUP: u32 = 61658;
/// ## [thumbsdown](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/thumbsdown.svg)

	pub const THUMBSDOWN: u32 = 61659;
/// ## [desktop-download](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/desktop-download.svg)

	pub const DESKTOPDOWNLOAD: u32 = 61660;
/// ## [beaker](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/beaker.svg)

	pub const BEAKER: u32 = 61661;
/// ## [bell](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/bell.svg)

	pub const BELL: u32 = 61662;
/// ## [watch](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/watch.svg)

	pub const WATCH: u32 = 61664;
/// ## [shield](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/shield.svg)

	pub const SHIELD: u32 = 61665;
/// ## [bold](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/bold.svg)

	pub const BOLD: u32 = 61666;
/// ## [text-size](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/text-size.svg)

	pub const TEXTSIZE: u32 = 61667;
/// ## [italic](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/italic.svg)

	pub const ITALIC: u32 = 61668;
/// ## [tasklist](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/tasklist.svg)

	pub const TASKLIST: u32 = 61669;
/// ## [verified](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/verified.svg)

	pub const VERIFIED: u32 = 61670;
/// ## [smiley](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/smiley.svg)

	pub const SMILEY: u32 = 61671;
/// ## [unverified](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/unverified.svg)

	pub const UNVERIFIED: u32 = 61672;
/// ## [device-desktop](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/device-desktop.svg)

	pub const DEVICEDESKTOP: u32 = 62076;
	pub const ALL_ICONS: [(&'static str, u32); 167] = [("HEART", 9829),("ZAP", 9889),("LIGHTBULB", 61440),("REPO", 61441),("REPOFORKED", 61442),("REPOPUSH", 61445),("REPOPULL", 61446),("BOOK", 61447),("OCTOFACE", 61448),("GITPULLREQUEST", 61449),("MARKGITHUB", 61450),("CLOUDDOWNLOAD", 61451),("CLOUDUPLOAD", 61452),("KEYBOARD", 61453),("GIST", 61454),("FILECODE", 61456),("FILETEXT", 61457),("FILEMEDIA", 61458),("FILEZIP", 61459),("FILEPDF", 61460),("TAG", 61461),("FILEDIRECTORY", 61462),("FILESUBMODULE", 61463),("PERSON", 61464),("JERSEY", 61465),("GITCOMMIT", 61471),("GITBRANCH", 61472),("GITMERGE", 61475),("MIRROR", 61476),("ISSUEOPENED", 61478),("ISSUEREOPENED", 61479),("ISSUECLOSED", 61480),("STAR", 61482),("COMMENT", 61483),("QUESTION", 61484),("ALERT", 61485),("SEARCH", 61486),("GEAR", 61487),("RADIOTOWER", 61488),("TOOLS", 61489),("SIGNOUT", 61490),("ROCKET", 61491),("RSS", 61492),("CLIPPY", 61493),("SIGNIN", 61494),("ORGANIZATION", 61495),("DEVICEMOBILE", 61496),("UNFOLD", 61497),("CHECK", 61498),("MAIL", 61499),("MAILREAD", 61500),("ARROWUP", 61501),("ARROWRIGHT", 61502),("ARROWDOWN", 61503),("ARROWLEFT", 61504),("PIN", 61505),("GIFT", 61506),("GRAPH", 61507),("TRIANGLELEFT", 61508),("CREDITCARD", 61509),("CLOCK", 61510),("RUBY", 61511),("BROADCAST", 61512),("KEY", 61513),("REPOFORCEPUSH", 61514),("REPOCLONE", 61516),("DIFF", 61517),("EYE", 61518),("COMMENTDISCUSSION", 61519),("MAILREPLY", 61521),("PRIMITIVEDOT", 61522),("PRIMITIVESQUARE", 61523),("DEVICECAMERA", 61526),("DEVICECAMERAVIDEO", 61527),("PENCIL", 61528),("INFO", 61529),("TRIANGLERIGHT", 61530),("TRIANGLEDOWN", 61531),("LINK", 61532),("PLUS", 61533),("THREEBARS", 61534),("CODE", 61535),("LOCATION", 61536),("LISTUNORDERED", 61537),("LISTORDERED", 61538),("QUOTE", 61539),("VERSIONS", 61540),("CALENDAR", 61544),("LOCK", 61546),("DIFFADDED", 61547),("DIFFREMOVED", 61548),("DIFFMODIFIED", 61549),("DIFFRENAMED", 61550),("HORIZONTALRULE", 61552),("ARROWSMALLRIGHT", 61553),("MILESTONE", 61557),("CHECKLIST", 61558),("MEGAPHONE", 61559),("CHEVRONRIGHT", 61560),("BOOKMARK", 61563),("SETTINGS", 61564),("DASHBOARD", 61565),("HISTORY", 61566),("LINKEXTERNAL", 61567),("MUTE", 61568),("X", 61569),("CIRCLESLASH", 61572),("PULSE", 61573),("SYNC", 61575),("TELESCOPE", 61576),("GISTSECRET", 61580),("HOME", 61581),("STOP", 61583),("BUG", 61585),("LOGOGITHUB", 61586),("FILEBINARY", 61588),("DATABASE", 61590),("SERVER", 61591),("DIFFIGNORED", 61593),("ELLIPSIS", 61594),("NONEWLINE", 61596),("HUBOT", 61597),("ARROWSMALLUP", 61599),("ARROWSMALLDOWN", 61600),("ARROWSMALLLEFT", 61601),("CHEVRONUP", 61602),("CHEVRONDOWN", 61603),("CHEVRONLEFT", 61604),("TRIANGLEUP", 61610),("GITCOMPARE", 61612),("LOGOGIST", 61613),("FILESYMLINKFILE", 61616),("FILESYMLINKDIRECTORY", 61617),("SQUIRREL", 61618),("GLOBE", 61622),("UNMUTE", 61626),("MENTION", 61630),("PACKAGE", 61636),("BROWSER", 61637),("TERMINAL", 61640),("MARKDOWN", 61641),("DASH", 61642),("FOLD", 61644),("INBOX", 61647),("TRASHCAN", 61648),("PAINTCAN", 61649),("FLAME", 61650),("BRIEFCASE", 61651),("PLUG", 61652),("CIRCUITBOARD", 61654),("MORTARBOARD", 61655),("LAW", 61656),("THUMBSUP", 61658),("THUMBSDOWN", 61659),("DESKTOPDOWNLOAD", 61660),("BEAKER", 61661),("BELL", 61662),("WATCH", 61664),("SHIELD", 61665),("BOLD", 61666),("TEXTSIZE", 61667),("ITALIC", 61668),("TASKLIST", 61669),("VERIFIED", 61670),("SMILEY", 61671),("UNVERIFIED", 61672),("DEVICEDESKTOP", 62076),];
}