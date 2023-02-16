import { useDisplay } from 'vuetify'
export const useNavDrawer = () => {
  const { mobile } = useDisplay()
  const drawer = useState<boolean>('drawer', () => false)

  onMounted(() => {
    drawer.value = !mobile.value
  })

  const toggleDrawer = () => {
    drawer.value = !drawer.value
  }
  return {
    drawer,
    mobile,
    toggleDrawer,
  }
}
