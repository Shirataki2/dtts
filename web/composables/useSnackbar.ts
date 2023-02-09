export const useSnackbar = () => {
  type SnackbarOptions = {
    text: string
    color?: string
    timeout?: number
  }
  const snackbar = useState<SnackbarOptions | null>('snackbar', () => null)
  const showSnackbar = (options: SnackbarOptions | null) => {
    snackbar.value = options
  }
  return {
    snackbar,
    showSnackbar,
  }
}
