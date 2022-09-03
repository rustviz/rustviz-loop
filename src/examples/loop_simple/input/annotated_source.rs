fn main() {
  let <tspan data-hash="1">x</tspan> = <tspan class="fn" data-hash="0" hash="4">String::from</tspan>("hello");
  for <tspan data-hash="3">i</tspan> in 1..10 {
    <tspan class="fn" data-hash="0" hash="5">f</tspan>(<tspan data-hash="1">&amp;x</tspan>)
  }
}

fn <tspan class="fn" data-hash="0" hash="5">f</tspan>(<tspan data-hash="2">s</tspan> : &amp;String) {
  <tspan class="fn" data-hash="0" hash="6">println!</tspan>("{}", <tspan data-hash="2">*s</tspan>);
}