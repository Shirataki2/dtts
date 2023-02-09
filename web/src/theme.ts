import type { ThemeDefinition } from 'vuetify'
import colors from './palette'

export const customLightTheme: ThemeDefinition = {
  dark: false,
  colors: {
    background: colors.grey.lighten4,
    surface: colors.shades.white,
    primary: colors.lightBlue.lighten2,
    secondary: colors.lime.lighten2,
    success: colors.teal.lighten2,
    warning: colors.amber.lighten2,
    error: colors.red.lighten2,
    info: colors.cyan.lighten2,
    'on-background': colors.shades.black,
    'on-surface': colors.grey.darken4,
    'on-primary': colors.grey.darken4,
    'on-secondary': colors.grey.darken4,
    'on-success': colors.grey.lighten5,
    'on-warning': colors.grey.darken4,
    'on-error': colors.grey.lighten5,
    'on-info': colors.grey.lighten5,

    header: colors.shades.white,
  },
}

export const customDarkTheme: ThemeDefinition = {
  dark: true,
  colors: {
    background: colors.blueGrey.darken4,
    surface: colors.blueGrey.darken3,
    primary: colors.lightBlue.darken2,
    secondary: colors.purple.darken2,
    success: colors.lightGreen.darken1,
    warning: colors.amber.darken3,
    error: colors.red.darken1,
    info: colors.cyan.darken2,
    'on-background': colors.shades.white,
    'on-surface': colors.grey.lighten4,
    'on-primary': colors.grey.lighten4,
    'on-secondary': colors.grey.lighten4,
    'on-success': colors.shades.white,
    'on-warning': colors.shades.white,
    'on-error': colors.red.lighten5,
    'on-info': colors.shades.white,

    header: colors.blue.darken4,
  },
}

export default {
  defaultTheme: 'customDarkTheme',
  themes: {
    customLightTheme,
    customDarkTheme,
  },
}
