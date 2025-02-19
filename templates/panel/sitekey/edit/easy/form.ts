/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
import getFormUrl from "../../../../utils/getFormUrl";
import genJsonPayload from "../../../../utils/genJsonPayload";
import createError from "../../../../components/error";

import VIEWS from "../../../../views/v1/routes";

import { validate, FORM } from "../../add/novice/ts/form";

const SUBMIT_BTN = <HTMLButtonElement>(
  document.querySelector(".sitekey-form__submit")
);
const key = SUBMIT_BTN.dataset.sitekey;
const submit = async (e: Event) => {
  e.preventDefault();

  const formUrl = getFormUrl(FORM);
  const payload = {
    pattern: validate(e),
    key,
  };
  console.debug(`[form submition] json payload: ${JSON.stringify(payload)}`);

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    window.location.assign(VIEWS.viewSitekey(key));
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

const addSubmitEventListener = (): void =>
  FORM.addEventListener("submit", submit, true);

export default addSubmitEventListener;
