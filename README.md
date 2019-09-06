# gethostname.rs

[![Current release](https://img.shields.io/crates/v/gethostname.svg)][crates]
[![Documentation](https://docs.rs/gethostname/badge.svg)][docs]
[![License](https://img.shields.io/github/license/lunaryorn/gethostname.rs.svg)][license]
[![CI status](https://github.com/lunaryorn/gethostname.rs/workflows/CI/badge.svg)][ci]

[gethostname()][ghn] for all platforms.

[crates]: https://crates.io/crates/gethostname
[docs]: https://docs.rs/gethostname
[license]: https://github.com/lunaryorn/gethostname.rs/blob/master/LICENSE
[ci]: https://github.com/lunaryorn/gethostname.rs/actions
[ghn]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html

## Prior art

[hostname] also provides `gethostname()`, but is [no longer maintained][1] as of
2019.  This crate improves the [Windows implementation][2].

[hostname]: https://github.com/fengcen/hostname
[1]: https://github.com/fengcen/hostname/pull/4#issuecomment-455735989
[2]: https://github.com/fengcen/hostname/pull/4#issuecomment-433722692

## License

Copyright 2019 Sebastian Wiesner <sebastian@swsnr.de>

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at <http://www.apache.org/licenses/LICENSE-2.0>.

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
