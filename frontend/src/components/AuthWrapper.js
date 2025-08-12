
'use client';

import { AuthProvider } from '../context/auth-context';

export default function AuthWrapper({ children }) {
  return <AuthProvider>{children}</AuthProvider>;
}
