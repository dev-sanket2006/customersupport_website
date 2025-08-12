'use client';

import { useTheme } from '../context/theme-context';

export default function BodyWrapper({ children, fonts }) {
  const { theme } = useTheme();

  return (
    <body
      className={`
        ${fonts}
        antialiased transition-colors
        ${theme === 'dark' ? 'bg-gray-900 text-white' : 'bg-white text-black'}
      `}
    >
      {children}
    </body>
  );
}
