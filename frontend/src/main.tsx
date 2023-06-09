import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider
} from "react-router-dom";
import { Header } from './Components/Ui/Header';
import ErrorPage from './ErrorPage';
import './index.css';
import { Home } from './routes/home/Home';
import { KeycardBase } from './routes/keycard/KeycardBase';
import { ManageKeycard } from './routes/keycard/ManageKeycard';
import { Login } from './routes/login/Login';
import { Main } from './routes/Main';
import { ChangeRequest } from './routes/request/ChangeRequest';
import { RequestBase } from './routes/request/RequestBase';
import { ShowAllRequestFromUser } from './routes/request/ShowAllRequestFromUser';
import { ShowRequests } from './routes/request/ShowRequests';
import { ShowAllUsers } from './routes/users/ShowAllUsers';

import i18next from 'i18next';
import UserContext, { IUserContext } from './context/UserContext';
import { GlobalKeycardList } from './routes/keycard/GlobalKeycardList';
import { Logs } from './routes/logs/Logs';
import { CreateKeycardRequest, CreateRoomRequest, CreateTempRequest, RequestPicker } from './routes/request/CreateRequest';
import { UseKeycard } from './routes/use-keycard/UseKeycard';
import { SelfUser, UserByUserId } from './routes/user/User';
import { UserBase } from './routes/user/UserBase';
import { LoadingProvider } from './util/Provider/LoadingProvider';
import { Rest } from './util/Rest';
import { decodeToken } from './util/token';
import { SidebarProvider } from './util/Provider/SidebarProvider';
import { Sidebar } from './Components/Ui/Sidebar';

const router = createBrowserRouter([
  {
    path: "/",
    element: <>
      <Sidebar />
      <Header />
    </>,
    children: [
      {
        path: "/use-keycard",
        element: <UseKeycard />
      },
      {
        path: "/",
        element: <Main />,
        errorElement: <ErrorPage />,
      },
      {
        path: "/login",
        element: <Login />,
      },
      {
        path: "/home",
        element: <Home />,
      },
      {
        path: "user",
        element: <UserBase />,
        children: [
          {
            path: "",
            element: <SelfUser />
          },
          {
            path: ":userId/request",
            element: <ShowAllRequestFromUser />
          },
          {
            path: ":userId",
            element: <UserByUserId />
          }
        ]
      },
      {
        path: "request",
        element: <ShowRequests />
      },
      {
        path: "request",
        element: <RequestBase />,
        children: [
          {
            path: "add-request",
            element: <RequestPicker />
          },
          {
            path: "add-request/room",
            element: <CreateRoomRequest />
          },
          {
            path: "add-request/keycard",
            element: <CreateKeycardRequest />
          },
          {
            path: "add-request/temp",
            element: <CreateTempRequest />
          },
          {
            path: "change-request/:requestId",
            element: <ChangeRequest />

          },
        ]
      },
      {
        path: "keycard",
        children: [
          {
            path: "",
            element: <GlobalKeycardList />
          },
          {
            path: "change-request/:requestId",
            element: < KeycardBase />,
            children: [
              {
                path: "",
                element: <ManageKeycard />
              }
            ]
          },
        ]
      },
      {
        path: "users",
        element: <ShowAllUsers />
      },
      {
        path: "logs",
        element: <Logs />
      },
    ]
  }

]);
const loadI18n = async () => {
  const supportedLanguages = ["de", "en"]
  const ressoucesBundles = await Promise.all(
    supportedLanguages.map(val => Rest.getRessourceBundle(val))
  )
  for (let idx = 0; idx < ressoucesBundles.length; idx++) {
    i18next.addResourceBundle(supportedLanguages[idx], "1", ressoucesBundles[idx])
  }
}
const queryClient = new QueryClient({ defaultOptions: { queries: { refetchInterval: 60000 } } })
const Default: React.FC<DefaultProps> = (props) => {
  const [ressourcesBundlesLoaded, setRessourcesBundlesLoaded] = React.useState(false);
  const callback = () => {
    const params = new window.URLSearchParams(document.cookie)

    if (params.has("token") && (localStorage.getItem("save_token") === 'true' || sessionStorage.getItem("save_token") === "true")) {
      if (params.get("token")) {
        const token = decodeToken(params);
        setjwt({ ...token, loggedIn: true })
      }
    } else {
      if (!(window.location.pathname === "/" || window.location.pathname === "/login")) {
        window.location.pathname = "/"

      }
    }
  }
  React.useEffect(() => {
    callback()
    loadI18n().then(() => {
      setRessourcesBundlesLoaded(true)
    })
    return () => {
    }
  }, []);
  const [jwt, setjwt] = React.useState({ loggedIn: false } as IUserContext);
  return (<>
    <UserContext.Provider value={{ ...jwt, set: setjwt }}>
      {ressourcesBundlesLoaded && props.children}
    </UserContext.Provider>
  </>)
}
if (!localStorage.getItem("language")) {
  localStorage.setItem("language", "en")
}

i18next.init({ defaultNS: '1', resources: {}, lng: localStorage.getItem("language") as string });

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SidebarProvider>
      <Default>
      <QueryClientProvider client={queryClient}>
        <LoadingProvider>
          <RouterProvider router={router} />
        </LoadingProvider>
      </QueryClientProvider>
    </Default>
    </SidebarProvider>

  </React.StrictMode>
);

interface DefaultProps {
  children: any;
}
