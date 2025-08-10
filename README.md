# fwl
Quick tool for restoring iptables rules.

---

![GitHub Release](https://img.shields.io/github/v/release/glcomtech/fwl?style=flat-square&logo=github)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/glcomtech/fwl/rust.yml?style=flat-square&logo=github)
![GitHub Repo stars](https://img.shields.io/github/stars/glcomtech/fwl?style=flat-square&logo=github)

![GitHub contributors](https://img.shields.io/github/contributors/glcomtech/fwl?style=flat-square&logo=github) ![GitHub last commit](https://img.shields.io/github/last-commit/glcomtech/fwl?style=flat-square&logo=github)

![GitHub License](https://img.shields.io/github/license/glcomtech/fwl?style=flat-square&logo=github)

---

## How to use

To use the program download the latest version here: https://github.com/glcomtech/fwl/releases

Unpack the archive:
```
tar -xzf fwl_0.2.1_x86_x64.tar.gz
```
Then enter the program directory:
```
cd fwl_0.2.1_x86_x64
```
And finally run the program:
```
sudo ./fwl
```

---

All set! Your iptables rules are restored!

---

If you want to build the program, clone the repository then:
```
cd fwl
```
Build the binary:
```
cargo build -r
```
If you need to use debugger:
```
cargo build
```

---

## Documentation
https://glcomtech.github.io/fwl/fwl/index.html

---

## ⚠️ LICENSE ⚠️

    fwl - quick tool for restoring iptables rules
    Copyright (C) 2025  Andrew Kushyk
    
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.
    
    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

---
