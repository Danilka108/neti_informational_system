import { cssBundleHref } from "@remix-run/css-bundle";
import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import { Box, Tab, Typography, Tabs, Stack, Paper, Divider, Button, Dialog, DialogTitle, DialogContent, DialogContentText, DialogActions, Fab } from "@mui/material";

export const API_HOST = "http://127.0.0.1:4000";

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import React, { useState } from "react";

export const links = () => [
  ...(cssBundleHref ? [{ rel: "stylesheet", href: cssBundleHref }] : []),
];

export default function App() {
  const [openDoc, setOpenDoc] = useState(false);

  const handleOpen = () => {
    setOpenDoc(true);
  }

  const handleClose = () => {
    setOpenDoc(false);
  }

  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body style={{ height: "100vh", margin: 0 }}>
        <Dialog open={openDoc} onClose={handleClose}>
          <DialogTitle>Operational documentation</DialogTitle>
          <DialogContent>
            <DialogContentText>
              Эксплуатационная документация на программное средство
              Основание: ГОСТ Р ИСО 9127 - 94
              Общие сведения о программном средстве
              Имя – ИС «информационная система вуза»
              Дата выпуска 27.12.2023
              Организация, поставляющая программное средство ФГБОУ ВО НГТУ
              Язык описания интерфейса пользователя – английский
              Язык описания документации – русский
              Принцип функционирования
              Функции, реализованные в информационной системе:
              Пользователь системы доступ к информации университетом, за исключением конфиденциальной.Администратор вуза имеет возможность добавлять / удалять / редактировать подразделения, учебные группы, учебные программы, добавлять / удалять физ.лиц из системы и редактировать их персональные данные.Администратор системы имеет возможность добавлять / удалять вузы.
              Диалоговый режим работы
              Ограничения на совместимость «информационная система вуза»:
              ОС Windows с поддержкой.NET;
              Процессор Intel Core i3 6100 или лучше;
              4 ГБ RAM;
              100 МБ дискового пространства;
              Печатающее устройство.
              Комплектность
              Исполняемый файл программы и все необходимые для запуска библиотеки;
              Программная документация.
              Акт о приемке
              Гарантийные обязательства представителя
              Гарантийное обслуживание производится в течение всего срока эксплуатации программного средства до вывода из эксплуатации.
              Адрес обслуживания потребителя:
              630087, Новосибирская обл., г.Новосибирск, ул.Немировича - Данченко, 23
              Тел / факс: 8(383) 765 - 43 - 21, 12 - 34 - 56
            </DialogContentText>
          </DialogContent>
          <DialogActions>
            <Button onClick={handleClose} color="primary">
              Close
            </Button>
          </DialogActions>
        </Dialog>
        <Button style={{ padding: "0", paddingTop: "5px" }} onClick={handleOpen}>docs</Button>
        <Outlet />
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}
