import { createTheme, CssBaseline, ThemeProvider } from '@mui/material';
import { createContext, useMemo } from 'react';
import {
  createHashRouter,
  createRoutesFromElements,
  Route,
  RouterProvider,
} from 'react-router-dom';
import { useLocalStorage } from 'usehooks-ts';
import Login from './pages/Login';
import Wallet from './pages/Wallet';

const router = createHashRouter(
  createRoutesFromElements(
    <Route path="/" element={<Login />}>
      <Route path="wallet" element={<Wallet />} />
    </Route>,
  ),
);

export interface DarkModeContext {
  toggle: () => void;
  isDark: boolean;
}

export const DarkModeContext = createContext<DarkModeContext>({
  toggle: () => {},
  isDark: false,
});

export default function App() {
  const [dark, setDark] = useLocalStorage('dark', false);

  const darkMode = useMemo<DarkModeContext>(
    () => ({
      toggle: () => setDark((dark) => !dark),
      isDark: dark,
    }),
    [dark, setDark],
  );

  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: dark ? 'dark' : 'light',
        },
      }),
    [dark],
  );

  return (
    <DarkModeContext.Provider value={darkMode}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <RouterProvider router={router} />
      </ThemeProvider>
    </DarkModeContext.Provider>
  );
}