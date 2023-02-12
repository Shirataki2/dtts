export const useValidate = (name: string) => {
  const validate = useState<() => Promise<boolean>>(`${name}_validator`, () => async () => false)
  const saving = useState<boolean>(`${name}_saving`, () => false)
  return {
    validate,
    saving,
  }
}
