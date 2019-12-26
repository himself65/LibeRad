const base = require('@secto-sessions/dev/config/eslint.js')
const { react } = require('@secto-sessions/dev/config/rules')

module.exports = {
  ...base,
  'parserOptions': {
    'ecmaFeatures': {
      'jsx': true
    },
    'useJSXTextNode': true,
    'project': './tsconfig.dev.json'
  },
  rules: {
    ...base.rules,
    ...react
  }
}
