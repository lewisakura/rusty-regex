import test from 'ava'

import { isValidRegex, matches } from '../index.js'

test('regex', (t) => {
  t.true(isValidRegex('a'))
  t.false(isValidRegex('['))

  t.true(matches('a', 'a'))

  t.false(matches('a', 'b'))
})
