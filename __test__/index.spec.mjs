import test from 'ava'

import { isValidRegex, matches, parseTemplate } from '../index.js'

test('regex', (t) => {
  t.true(isValidRegex('a'))
  t.false(isValidRegex('['))

  t.true(matches('a', 'a'))

  t.false(matches('a', 'b'))
})

test('regex templates', (t) => {
  t.deepEqual(parseTemplate('(a)b', 'ab', '${1}b'), 'ab');
  t.deepEqual(parseTemplate("(?P<named>a)b", "ab", "${named}b"), "ab");
  t.deepEqual(parseTemplate("(?P<named>a)b", "ab", "${1}b"), "ab");
})