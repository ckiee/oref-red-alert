# oref-red-alert

Find out if missiles are going to hit Israel from inside Rust using the [Home Front Command API](https://www.oref.org.il//12481-en/Pakar.aspx).

## Usage

```toml
[dependencies]
oref-red-alert = "1.0"
```

```rust
let alert = Alert::get().unwrap();
if alert.is_some() {
    println!("Affected cities: {}", alert.unwrap().areas.len())
} else {
    println!("There is no alert available");
}
```

## License

This library is licensed under the MIT license; the terms of the license can be found in the `LICENSE` file.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
