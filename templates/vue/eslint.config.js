import antfu from '@antfu/eslint-config'

export default antfu({
  formatters: true,
  vue: true,
  stylistic: true,
  typescript: true,
  ignores: [
    'src/assets/**/*',
    'src-tauri/**/*',
    'src/components/ui/**/*',
    'src/*.d.ts',
  ],
  settings: {
    'import/core-modules': ['vue-router/auto', 'vue-router/auto-routes'],
  },
})
