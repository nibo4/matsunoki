import { I18nContext, useI18n } from "@solid-primitives/i18n";
import type { Component } from "solid-js";
import { DefaultLayout } from "../../layout/default";
import GoogleLogo from "../../assets/btn_google_dark_normal_ios.svg";
import { SignUpI18nContext } from "./dict";
import styles from "./style.module.css";

const PageContent: Component = () => {
  const [t] = useI18n();
  return (
    <DefaultLayout>
      <div class={styles["container"]}>
        <article class={styles["sign-up-card"]}>
          <section>
            <h3>{t("signUp")}</h3>
            <label>{t("description")}</label>
          </section>
          <section>
            <button class={styles["sign-up-with-google"]}>
              <GoogleLogo />
              <span>{t("signUpWithGoogle")}</span>
            </button>
          </section>
        </article>
      </div>
    </DefaultLayout>
  );
};

export const SignUpPage: Component = () => {
  return (
    <I18nContext.Provider value={SignUpI18nContext}>
      <PageContent />
    </I18nContext.Provider>
  );
};
