import { Everest } from './everest';

const script = `fn rec(x) {
  print "hi";
  if (x >= 0) {
    print x;
    return rec(x - 1);
  }
}

rec(10);`;

Everest.run(script);
