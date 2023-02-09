import { useUser } from '@/composables/useUser'

export default defineNuxtRouteMiddleware((to, _from) => {
  const { isLoggedin } = useUser()
  if (!isLoggedin.value) {
    return '/login'
  }
})
