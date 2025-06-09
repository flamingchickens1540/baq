// place files you want to import through the `$lib` alias in this folder.
function assert(condition, message) {
  if (!condition) {
    throw message || "Assertion failed"
  }
}
