import eslintPluginSvelte from 'eslint-plugin-svelte';
import svelteConfig from './svelte.config.js';
export default [
  {
    extends: [
      'eslint:recommend',
      'plugin:svelte/recommended'
    ],
    parser: '@typescript-eslint/parser',
    parserOptions: {
      extraFileExtensions: ['.svelte'] // This is a required setting in `@typescript-eslint/parser` v4.24.0.
    },
    overrides: [
      {
        files: ['*.svelte'],
        parser: 'svelte-eslint-parser',
        // Parse the `<script>` in `.svelte` as TypeScript by adding the following configuration.
        parserOptions: {
          parser: '@typescript-eslint/parser'
        }
      },
    ],
    languageOptions: {
      parserOptions: {
        // Specify the `svelte.config.js`.
        svelteConfig
      }
    }
  }
];
