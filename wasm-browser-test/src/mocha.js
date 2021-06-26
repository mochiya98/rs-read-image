import mocha from "mocha/mocha";
import "mocha/mocha.css";

export function runMocha(suite) {
  const container = document.createElement("div");
  container.id = "mocha";
  document.body.appendChild(container);
  mocha.setup("bdd");
  mocha.checkLeaks();
  suite();
  mocha.run();
}
