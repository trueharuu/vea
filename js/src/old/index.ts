import { Everest } from './everest';

const script = `fn p(x) { print x; return 2 * x; }
var i = 0;
print p(i);`;

Everest.run(script);
