# CI pipeline
It consists of multiple stages below, each having one or more jobs
Latest CI pipeline of master branch can be seen at [https://gitlab.com/satoshilabs/trezor/trezor-firmware/-/pipelines/master/latest](https://gitlab.com/satoshilabs/trezor/trezor-firmware/-/pipelines/master/latest)
## ENVIRONMENT stage - [file](../ci/environment.yml)

Connected with creating the testing image for CI

Consists of **2 jobs** below:
- ### [environment manual](https://github.com/trezor/trezor-firmware/blob/master/ci/environment.yml#L31)
Missing description
```sh
No script defined, probably extends parent
```
- ### [environment scheduled](https://github.com/trezor/trezor-firmware/blob/master/ci/environment.yml#L35)
Missing description
```sh
No script defined, probably extends parent
```
---
## PREBUILD stage - [file](../ci/prebuild.yml)

Static checks on the code.

Consists of **7 jobs** below:
- ### [style prebuild](https://github.com/trezor/trezor-firmware/blob/master/ci/prebuild.yml#L16)
Check the code for style correctness and perform some static code analysis.
Biggest part is the python one - using `flake8`, `isort`, `black`, `pylint` and `pyright`,
also checking Rust files by `rustftm` and C files by `clang-format`.
Changelogs formats are checked
```sh
nix-shell --run "poetry run make style_check"
```
- ### [common prebuild](https://github.com/trezor/trezor-firmware/blob/master/ci/prebuild.yml#L25)
Check validity of coin definitions and protobuf files
```sh
nix-shell --run "poetry run make defs_check"
```
- ### [gen prebuild](https://github.com/trezor/trezor-firmware/blob/master/ci/prebuild.yml#L32)
Check validity of auto-generated files
```sh
nix-shell --run "poetry run make gen_check"
```
- ### [editor prebuild](https://github.com/trezor/trezor-firmware/blob/master/ci/prebuild.yml#L39)
Checking format of .editorconfig files
```sh
nix-shell --run "make editor_check"
```
- ### [yaml prebuild](https://github.com/trezor/trezor-firmware/blob/master/ci/prebuild.yml#L46)
All .yml/.yaml files are checked for syntax validity and other correctness
```sh
nix-shell --run "poetry run make yaml_check"
```
- ### [release commit messages prebuild](https://github.com/trezor/trezor-firmware/blob/master/ci/prebuild.yml#L53)
Checking the format of release commit messages
```sh
nix-shell --run "ci/check_release_commit_messages.sh"
```
- ### [changelog prebuild](https://github.com/trezor/trezor-firmware/blob/master/ci/prebuild.yml#L70)
Verifying that all commits changing some functionality have a changelog entry
or contain `[no changelog]` in the commit message
```sh
nix-shell --run "ci/check_changelog.sh"
```
---
## BUILD stage - [file](../ci/build.yml)

All builds are published as artifacts so they can be downloaded and used.

Consists of **27 jobs** below:
- ### [core fw regular build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L20)
Build of Core into firmware. Regular version.
**Are you looking for Trezor T firmware build? This is most likely it.**
```sh
nix-shell --run "poetry run make -C core build_boardloader"
nix-shell --run "poetry run make -C core build_bootloader"
nix-shell --run "poetry run make -C core build_bootloader_ci"
nix-shell --run "poetry run make -C core build_prodtest"
nix-shell --run "poetry run make -C core build_firmware"
nix-shell --run "poetry run make -C core sizecheck"
cp core/build/firmware/firmware.bin trezor-fw-regular-$CORE_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [core fw regular debug build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L41)
Build of Core into firmware with enabled _debug_ mode. In debug mode you can
upload mnemonic seed, use debug link etc. which enables device tests. Storage
on the device gets wiped on every start in this firmware.
```sh
nix-shell --run "PYOPT=0 poetry run make -C core build_firmware"
cp core/build/firmware/firmware.bin trezor-fw-regular-debug-$CORE_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [core fw regular production build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L54)
Missing description
```sh
nix-shell --run "poetry run make -C core build_boardloader"
nix-shell --run "poetry run make -C core build_bootloader"
nix-shell --run "poetry run make -C core build_bootloader_ci"
nix-shell --run "poetry run make -C core build_prodtest"
nix-shell --run "poetry run make -C core build_firmware"
nix-shell --run "poetry run make -C core sizecheck"
cp core/build/firmware/firmware.bin trezor-fw-regular-production-$CORE_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [core fw btconly build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L77)
Build of Core into firmware. Bitcoin-only version.
```sh
nix-shell --run "poetry run make -C core build_firmware"
mv core/build/firmware/firmware.bin core/build/firmware/firmware-bitcoinonly.bin
nix-shell --run "poetry run ./tools/check-bitcoin-only core/build/firmware/firmware-bitcoinonly.bin"
cp core/build/firmware/firmware-bitcoinonly.bin trezor-fw-btconly-$CORE_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [core fw btconly debug build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L94)
Missing description
```sh
nix-shell --run "poetry run make -C core build_firmware"
cp core/build/firmware/firmware.bin trezor-fw-btconly-debug-$CORE_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [core fw btconly production build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L117)
Missing description
```sh
nix-shell --run "poetry run make -C core build_firmware"
nix-shell --run "poetry run ./tools/check-bitcoin-only core/build/firmware/firmware.bin"
cp core/build/firmware/firmware.bin trezor-fw-btconly-production-$CORE_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [core fw btconly t1 build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L136)
Missing description
```sh
nix-shell --run "poetry run make -C core build_firmware"
cp core/build/firmware/firmware.bin trezor-fw-btconly-t1-$CORE_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [core unix regular build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L154)
Non-frozen emulator build. This means you still need Python files
present which get interpreted.
```sh
nix-shell --run "poetry run make -C core build_unix"
```
- ### [core unix regular asan build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L166)
Missing description
```sh
nix-shell --run "poetry run make -C core build_unix"
```
- ### [core unix frozen regular build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L185)
Build of Core into UNIX emulator. Something you can run on your laptop.
Frozen version. That means you do not need any other files to run it,
it is just a single binary file that you can execute directly.
```sh
nix-shell --run "poetry run make -C core build_unix_frozen"
```
- ### [core unix frozen btconly debug build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L202)
Build of Core into UNIX emulator. Something you can run on your laptop.
Frozen version. That means you do not need any other files to run it,
it is just a single binary file that you can execute directly.
See [Emulator](../core/emulator/index.md) for more info.
Debug mode enabled, Bitcoin-only version.
```sh
nix-shell --run "poetry run make -C core build_unix_frozen"
mv core/build/unix/trezor-emu-core core/build/unix/trezor-emu-core-bitcoinonly
```
- ### [core unix frozen btconly debug asan build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L218)
Missing description
```sh
nix-shell --run "poetry run make -C core build_unix_frozen"
mv core/build/unix/trezor-emu-core core/build/unix/trezor-emu-core-bitcoinonly
```
- ### [core unix frozen debug build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L241)
Build of Core into UNIX emulator. Something you can run on your laptop.
Frozen version. That means you do not need any other files to run it,
it is just a single binary file that you can execute directly.
**Are you looking for a Trezor T emulator? This is most likely it.**
```sh
nix-shell --run "poetry run make -C core build_unix_frozen"
```
- ### [core unix frozen debug asan build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L254)
Missing description
```sh
nix-shell --run "poetry run make -C core build_unix_frozen"
```
- ### [core unix frozen debug build arm](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L270)
Missing description
```sh
nix-shell --run "poetry run make -C core build_unix_frozen"
mv core/build/unix/trezor-emu-core core/build/unix/trezor-emu-core-arm
```
- ### [core unix frozen btconly debug t1 build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L291)
Missing description
```sh
nix-shell --run "poetry run make -C core build_unix_frozen"
mv core/build/unix/trezor-emu-core core/build/unix/trezor-emu-core-bitcoinonly
```
- ### [core macos frozen regular build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L306)
Missing description
```sh
nix-shell --option system x86_64-darwin --run "poetry run make -C core build_unix_frozen"
export NAME="trezor-emu-core.darwin"
cp -v core/build/unix/trezor-emu-core ./$NAME
chmod +x $NAME
echo '"$(dirname "$BASH_SOURCE")"/trezor-emu-core.darwin' > trezor-emulator.command
chmod u+x trezor-emulator.command
```
- ### [crypto build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L331)
Build of our cryptographic library, which is then incorporated into the other builds.
```sh
cp -r crypto crypto_noasan
nix-shell --run "poetry run make -C crypto"
nix-shell --run "export ADDRESS_SANITIZER=0; poetry run make -C crypto_noasan"
mv crypto_noasan/tests/test_check crypto/tests/test_check_noasan
```
- ### [legacy fw regular build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L360)
Missing description
```sh
nix-shell --run "export PRODUCTION=1 && poetry run legacy/script/cibuild"
nix-shell --run "poetry run legacy/script/setup"
nix-shell --run "export PRODUCTION=0 && poetry run legacy/script/cibuild"
nix-shell --run "poetry run make -C legacy/demo"
mv legacy/firmware/trezor.bin trezor-fw-regular-$LEGACY_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [legacy fw regular debug build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L376)
Missing description
```sh
nix-shell --run "export PRODUCTION=1 && poetry run legacy/script/cibuild"
nix-shell --run "poetry run legacy/script/setup"
nix-shell --run "export PRODUCTION=0 && poetry run legacy/script/cibuild"
mv legacy/firmware/trezor.bin trezor-fw-regular-debug-$LEGACY_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [legacy fw btconly build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L393)
Missing description
```sh
nix-shell --run "export PRODUCTION=1 && poetry run legacy/script/cibuild"
nix-shell --run "poetry run legacy/script/setup"
nix-shell --run "export PRODUCTION=0 && poetry run legacy/script/cibuild"
mv legacy/firmware/trezor.bin legacy/firmware/trezor-bitcoinonly.bin
nix-shell --run "poetry run ./tools/check-bitcoin-only legacy/firmware/trezor-bitcoinonly.bin"
mv legacy/firmware/trezor-bitcoinonly.bin trezor-fw-btconly-$LEGACY_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [legacy fw btconly debug build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L412)
Missing description
```sh
nix-shell --run "export PRODUCTION=1 && poetry run legacy/script/cibuild"
nix-shell --run "poetry run legacy/script/setup"
nix-shell --run "export PRODUCTION=0 && poetry run legacy/script/cibuild"
nix-shell --run "poetry run ./tools/check-bitcoin-only legacy/firmware/trezor.bin"
mv legacy/firmware/trezor.bin trezor-fw-btconly-debug-$LEGACY_VERSION-$CI_COMMIT_SHORT_SHA.bin
```
- ### [legacy emu regular debug build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L433)
Regular version (not only Bitcoin) of above.
**Are you looking for a Trezor One emulator? This is most likely it.**
```sh
nix-shell --run "poetry run legacy/script/cibuild"
```
- ### [legacy emu regular debug asan build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L448)
Missing description
```sh
nix-shell --run "poetry run legacy/script/cibuild"
```
- ### [legacy emu regular debug build arm](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L466)
Missing description
```sh
nix-shell --run "poetry run legacy/script/cibuild"
mv legacy/firmware/trezor.elf  legacy/firmware/trezor-arm.elf
```
- ### [legacy emu btconly debug build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L491)
Build of Legacy into UNIX emulator. Use keyboard arrows to emulate button presses.
Bitcoin-only version.
```sh
nix-shell --run "poetry run legacy/script/cibuild"
mv legacy/firmware/trezor.elf legacy/firmware/trezor-bitcoinonly.elf
```
- ### [legacy emu btconly debug asan build](https://github.com/trezor/trezor-firmware/blob/master/ci/build.yml#L508)
Missing description
```sh
nix-shell --run "poetry run legacy/script/cibuild"
mv legacy/firmware/trezor.elf legacy/firmware/trezor-bitcoinonly.elf
```
---
## TEST stage - [file](../ci/test.yml)

All the tests run test cases on the freshly built emulators from the previous `BUILD` stage.

Consists of **32 jobs** below:
- ### [core unit test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L15)
Python and rust unit tests, checking TT functionality.
```sh
nix-shell --run "poetry run make -C core test | ts -s"
nix-shell --run "poetry run make -C core test_rust | ts -s"
nix-shell --run "poetry run make -C core clippy | ts -s"
```
- ### [core unit test asan](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L25)
Missing description
```sh
nix-shell --run "poetry run make -C core test | ts -s"
nix-shell --run "poetry run make -C core test_rust | ts -s"
nix-shell --run "poetry run make -C core clippy | ts -s"
```
- ### [core unit test t1](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L41)
Missing description
```sh
nix-shell --run "poetry run make -C core test_rust | ts -s"
```
- ### [core device test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L56)
Device tests for Core. Running device tests and also comparing screens
with the expected UI result.
See artifacts for a comprehensive report of UI.
See [docs/tests/ui-tests](../docs/tests/ui-tests.md) for more info.
```sh
nix-shell --run "poetry run make -C core test_emu_ui | ts -s"
mv core/src/.coverage core/.coverage.test_emu
```
- ### [core device asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L85)
Missing description
```sh
nix-shell --run "poetry run make -C core test_emu | ts -s"
```
- ### [core btconly device test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L102)
Device tests excluding altcoins, only for BTC.
```sh
nix-shell --run "poetry run make -C core test_emu | ts -s"
```
- ### [core btconly device asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L122)
Missing description
```sh
nix-shell --run "poetry run make -C core test_emu | ts -s"
```
- ### [core monero test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L142)
Monero tests.
```sh
nix-shell --arg fullDeps true --run "poetry run make -C core test_emu_monero | ts -s"
mv core/src/.coverage core/.coverage.test_emu_monero
```
- ### [core monero asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L161)
Missing description
```sh
nix-shell --arg fullDeps true --run "poetry run make -C core test_emu_monero | ts -s"
mv core/src/.coverage core/.coverage.test_emu_monero
```
- ### [core u2f test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L183)
Tests for U2F and HID.
```sh
nix-shell --run "poetry run make -C tests/fido_tests/u2f-tests-hid | ts -s"
nix-shell --run "poetry run make -C core test_emu_u2f | ts -s"
mv core/src/.coverage core/.coverage.test_emu_u2f
```
- ### [core u2f asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L202)
Missing description
```sh
nix-shell --run "poetry run make -C tests/fido_tests/u2f-tests-hid | ts -s"
nix-shell --run "poetry run make -C core test_emu_u2f | ts -s"
```
- ### [core fido2 test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L220)
FIDO2 device tests.
```sh
pgrep trezor-emu-core || true
nix-shell --run "poetry run make -C core test_emu_fido2 | ts -s"
pgrep trezor-emu-core || true
mv core/src/.coverage core/.coverage.test_emu_fido2
```
- ### [core fido2 asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L243)
Missing description
```sh
pgrep trezor-emu-core || true
nix-shell --run "poetry run make -C core test_emu_fido2 | ts -s"
pgrep trezor-emu-core || true
```
- ### [core click test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L263)
Click tests.
See [docs/tests/click-tests](../docs/tests/click-tests.md) for more info.
```sh
nix-shell --run "poetry run make -C core test_emu_click | ts -s"
```
- ### [core click asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L280)
Missing description
```sh
nix-shell --run "poetry run make -C core test_emu_click | ts -s"
```
- ### [core upgrade test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L301)
Upgrade tests.
See [docs/tests/upgrade-tests](../docs/tests/upgrade-tests.md) for more info.
```sh
nix-shell --run "tests/download_emulators.sh"
nix-shell --run "poetry run pytest --junitxml=tests/junit.xml tests/upgrade_tests | ts -s"
```
- ### [core upgrade asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L320)
Missing description
```sh
nix-shell --run "tests/download_emulators.sh"
nix-shell --run "poetry run pytest --junitxml=tests/junit.xml tests/upgrade_tests | ts -s"
```
- ### [core persistence test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L342)
Persistence tests.
```sh
nix-shell --run "poetry run pytest --junitxml=tests/junit.xml tests/persistence_tests | ts -s"
```
- ### [core persistence asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L358)
Missing description
```sh
nix-shell --run "poetry run pytest --junitxml=tests/junit.xml tests/persistence_tests | ts -s"
```
- ### [core hwi test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L376)
Missing description
```sh
nix-shell --run "git clone https://github.com/bitcoin-core/HWI.git"
nix-shell --arg fullDeps true --run "cd HWI && poetry install && poetry run ./test/test_trezor.py --model_t ../core/build/unix/trezor-emu-core bitcoind"
```
- ### [crypto test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L393)
Missing description
```sh
./crypto/tests/aestst
./crypto/tests/test_check
./crypto/tests/test_openssl 1000
nix-shell --run "cd crypto && ITERS=10 poetry run pytest --junitxml=tests/junit.xml tests | ts -s"
nix-shell --run "CK_TIMEOUT_MULTIPLIER=20 valgrind -q --error-exitcode=1 ./crypto/tests/test_check_noasan | ts -s"
```
- ### [legacy device test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L424)
Missing description
```sh
nix-shell --run "poetry run make -C legacy test_emu_ui | ts -s"
```
- ### [legacy asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L451)
Missing description
```sh
nix-shell --run "poetry run make -C legacy test_emu | ts -s"
```
- ### [legacy btconly test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L463)
Missing description
```sh
nix-shell --run "poetry run make -C legacy test_emu | ts -s"
```
- ### [legacy btconly asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L483)
Missing description
```sh
nix-shell --run "poetry run make -C legacy test_emu | ts -s"
```
- ### [legacy upgrade test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L498)
Missing description
```sh
nix-shell --run "tests/download_emulators.sh"
nix-shell --run "poetry run pytest --junitxml=tests/junit.xml tests/upgrade_tests | ts -s"
```
- ### [legacy upgrade asan test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L517)
Missing description
```sh
nix-shell --run "tests/download_emulators.sh"
nix-shell --run "poetry run pytest --junitxml=tests/junit.xml tests/upgrade_tests | ts -s"
```
- ### [legacy hwi test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L538)
Missing description
```sh
nix-shell --run "git clone https://github.com/bitcoin-core/HWI.git"
nix-shell --arg fullDeps true --run "cd HWI && poetry install && poetry run ./test/test_trezor.py --model_1 ../legacy/firmware/trezor.elf bitcoind"
```
- ### [python test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L556)
Missing description
```sh
nix-shell --arg fullDeps true --run "unset _PYTHON_SYSCONFIGDATA_NAME && cd python && poetry run tox | ts -s"
```
- ### [storage test](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L585)
Missing description
```sh
nix-shell --run "poetry run make -C storage/tests build | ts -s"
nix-shell --run "poetry run make -C storage/tests tests_all | ts -s"
```
- ### [core unix memory profiler](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L608)
Missing description
```sh
nix-shell --run "poetry run make -C core build_unix_frozen | ts -s"
nix-shell --run "poetry run make -C core test_emu | ts -s"
nix-shell --run "mkdir core/prof/memperf-html"
nix-shell --run "poetry run core/tools/alloc.py --alloc-data=core/src/alloc_data.txt html core/prof/memperf-html"
```
- ### [connect test core](https://github.com/trezor/trezor-firmware/blob/master/ci/test.yml#L632)
Missing description
```sh
/trezor-user-env/run.sh &
nix-shell --run "tests/connect_tests/connect_tests.sh 2.99.99"
```
---
## TEST-HW stage - [file](../ci/test-hw.yml)

Consists of **5 jobs** below:
- ### [hardware core regular device test](https://github.com/trezor/trezor-firmware/blob/master/ci/test-hw.yml#L25)
[Device tests](../docs/tests/device-tests.md) that run against an actual physical Trezor T.
The device needs to have special bootloader, found in `core/embed/bootloader_ci`, that
makes it possible to flash firmware without confirmation on the touchscreen.

All hardware tests are run nightly on the `master` branch, as well as on push to branches
with whitelisted prefix. If you want hardware tests ran on your branch, make sure its
name starts with `hw/`.

Currently it's not possible to run all regular TT tests without getting into
a state where the micropython heap is too fragmented and allocations fail
(often manifesting as a stuck test case). For that reason some tests are
skipped.
See also: https://github.com/trezor/trezor-firmware/issues/1371
```sh
cd ci/hardware_tests
set -a
source hardware.cfg
set +a
nix-shell --run "cd ../.. && poetry install"
nix-shell --run "poetry run python bootstrap.py tt ../../trezor-*.bin | ts -s"
nix-shell --run "poetry run pytest ../../tests/device_tests | ts -s"
```
- ### [hardware core btconly device test](https://github.com/trezor/trezor-firmware/blob/master/ci/test-hw.yml#L54)
Also device tests on physical Trezor T but with Bitcoin-only firmware.
```sh
cd ci/hardware_tests
set -a
source hardware.cfg
set +a
nix-shell --run "cd ../.. && poetry install"
nix-shell --run "poetry run python bootstrap.py tt ../../trezor-*.bin | ts -s"
nix-shell --run "poetry run pytest ../../tests/device_tests | ts -s"
```
- ### [hardware core monero test](https://github.com/trezor/trezor-firmware/blob/master/ci/test-hw.yml#L83)
Missing description
```sh
cd ci/hardware_tests
set -a
source hardware.cfg
set +a
nix-shell --run "cd ../.. && poetry install"
nix-shell --run "poetry run python bootstrap.py tt ../../trezor-*.bin | ts -s"
nix-shell --arg fullDeps true --run "cd ../../core/tests && ./run_tests_device_emu_monero.sh $TESTOPTS | ts -s"
```
- ### [hardware legacy regular device test](https://github.com/trezor/trezor-firmware/blob/master/ci/test-hw.yml#L113)
[Device tests](../docs/tests/device-tests.md) executed on physical Trezor 1.
This works thanks to [tpmb](https://github.com/mmahut/tpmb), which is a small arduino
device capable of pushing an actual buttons on the device.
```sh
cd ci/hardware_tests
nix-shell --run "./t1_hw_test.sh | ts -s"
```
- ### [hardware legacy btconly device test](https://github.com/trezor/trezor-firmware/blob/master/ci/test-hw.yml#L137)
Also device tests on physical Trezor 1 but with Bitcoin-only firmware.
```sh
cd ci/hardware_tests
nix-shell --run "./t1_hw_test.sh | ts -s"
```
---
## POSTTEST stage - [file](../ci/posttest.yml)

Consists of **2 jobs** below:
- ### [core unix coverage posttest](https://github.com/trezor/trezor-firmware/blob/master/ci/posttest.yml#L10)
Missing description
```sh
nix-shell --run "poetry run make -C core coverage"
```
- ### [unix ui changes](https://github.com/trezor/trezor-firmware/blob/master/ci/posttest.yml#L31)
Missing description
```sh
nix-shell --run "cd tests/ui_tests ; poetry run python reporting/report_master_diff.py"
mv tests/ui_tests/reporting/reports/master_diff/ .
```
---
## DEPLOY stage - [file](../ci/deploy.yml)

Consists of **12 jobs** below:
- ### [release core fw regular deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L5)
Missing description
```sh
export VERSION=$(./tools/version.sh core/embed/firmware/version.h)
export NAME="trezor-fw-regular-$VERSION-$CI_COMMIT_SHORT_SHA.bin"
echo "Deploying to ${DEPLOY_DIRECTORY}/$NAME"
mkdir -p "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}"
rsync --delete -va $NAME "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}/$NAME"
```
- ### [release core fw btconly deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L26)
Missing description
```sh
export VERSION=$(./tools/version.sh core/embed/firmware/version.h)
export NAME="trezor-fw-btconly-$VERSION-$CI_COMMIT_SHORT_SHA.bin"
echo "Deploying to ${DEPLOY_DIRECTORY}/$NAME"
mkdir -p "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}"
rsync --delete -va $NAME "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}/$NAME"
```
- ### [release core fw regular debug deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L47)
Missing description
```sh
export VERSION=$(./tools/version.sh core/embed/firmware/version.h)
export NAME="trezor-fw-regular-debug-$VERSION-$CI_COMMIT_SHORT_SHA.bin"
echo "Deploying to ${DEPLOY_DIRECTORY}/$NAME"
mkdir -p "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}"
rsync --delete -va $NAME "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}/$NAME"
```
- ### [release core fw btconly debug deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L68)
Missing description
```sh
export VERSION=$(./tools/version.sh core/embed/firmware/version.h)
export NAME="trezor-fw-btconly-debug-$VERSION-$CI_COMMIT_SHORT_SHA.bin"
echo "Deploying to ${DEPLOY_DIRECTORY}/$NAME"
mkdir -p "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}"
rsync --delete -va $NAME "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}/$NAME"
```
- ### [release legacy fw regular deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L91)
Missing description
```sh
export VERSION=$(./tools/version.sh legacy/firmware/version.h)
export NAME="trezor-fw-regular-$VERSION-$CI_COMMIT_SHORT_SHA.bin"
echo "Deploying to ${DEPLOY_DIRECTORY}/$NAME"
mkdir -p "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}"
rsync --delete -va $NAME "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}/$NAME"
```
- ### [release legacy fw btconly deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L112)
Missing description
```sh
export VERSION=$(./tools/version.sh legacy/firmware/version.h)
export NAME="trezor-fw-btconly-$VERSION-$CI_COMMIT_SHORT_SHA.bin"
echo "Deploying to ${DEPLOY_DIRECTORY}/$NAME"
mkdir -p "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}"
rsync --delete -va $NAME "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}/$NAME"
```
- ### [release legacy fw regular debug deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L133)
Missing description
```sh
export VERSION=$(./tools/version.sh legacy/firmware/version.h)
export NAME="trezor-fw-regular-debug-$VERSION-$CI_COMMIT_SHORT_SHA.bin"
echo "Deploying to ${DEPLOY_DIRECTORY}/$NAME"
mkdir -p "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}"
rsync --delete -va $NAME "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}/$NAME"
```
- ### [release legacy fw btconly debug deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L154)
Missing description
```sh
export VERSION=$(./tools/version.sh legacy/firmware/version.h)
export NAME="trezor-fw-btconly-debug-$VERSION-$CI_COMMIT_SHORT_SHA.bin"
echo "Deploying to ${DEPLOY_DIRECTORY}/$NAME"
mkdir -p "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}"
rsync --delete -va $NAME "${DEPLOY_BASE_DIR}/${DEPLOY_DIRECTORY}/$NAME"
```
- ### [release core unix debug deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L177)
Missing description
```sh
export VERSION=$(./tools/version.sh core/embed/firmware/version.h)
DEST="$DEPLOY_PATH/trezor-emu-core-v$VERSION"
DEST_ARM="$DEPLOY_PATH/trezor-emu-core-arm-v$VERSION"
echo "Deploying to $DEST and $DEST_ARM"
nix-shell -p patchelf --run "patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 core/build/unix/trezor-emu-core"
nix-shell -p patchelf --run "patchelf --set-interpreter /lib/ld-linux-aarch64.so.1 core/build/unix/trezor-emu-core-arm"
rsync --delete -va core/build/unix/trezor-emu-core "$DEST"
rsync --delete -va core/build/unix/trezor-emu-core-arm "$DEST_ARM"
```
- ### [release legacy unix debug deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L202)
Missing description
```sh
export VERSION=$(./tools/version.sh legacy/firmware/version.h)
DEST="$DEPLOY_PATH/trezor-emu-legacy-v$VERSION"
DEST_ARM="$DEPLOY_PATH/trezor-emu-legacy-arm-v$VERSION"
echo "Deploying to $DEST and $DEST_ARM"
nix-shell -p patchelf --run "patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 legacy/firmware/trezor.elf"
nix-shell -p patchelf --run "patchelf --set-interpreter /lib/ld-linux-aarch64.so.1 legacy/firmware/trezor-arm.elf"
rsync --delete -va legacy/firmware/trezor.elf "$DEST"
rsync --delete -va legacy/firmware/trezor-arm.elf "$DEST_ARM"
```
- ### [ui tests fixtures deploy](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L229)
Missing description
```sh
echo "Deploying to $DEPLOY_PATH"
rsync --delete -va ci/ui_test_records/* "$DEPLOY_PATH"
```
- ### [common sync](https://github.com/trezor/trezor-firmware/blob/master/ci/deploy.yml#L246)
Missing description
```sh
echo "Synchronizing common with the trezor-common repository"
./ci/common_sync/common_repo_sync.sh
```
---
