import { I18nContext, useI18n } from "@solid-primitives/i18n";
import { Component, createEffect } from "solid-js";
import { DefaultLayout } from "../../layout/default";
import GoogleLogo from "../../assets/btn_google_dark_normal_ios.svg";
import { useCore } from "../../core-context";
import { SignUpI18nContext } from "./dict";
import styles from "./style.module.css";
import { useSignUpDIContext } from "./di-context";

const PageContent: Component = () => {
  const [t] = useI18n();
  const core = useCore();
  const { navigateToRoot } = useSignUpDIContext();

  createEffect(() => {
    const unsubscribe = core.readModels.signedInObservable.subscribe(
      (result) => {
        if (result.ok) navigateToRoot();
        if (result.err) window.alert("ログインに失敗しました");
      }
    );
    return unsubscribe;
  });

  return (
    <DefaultLayout>
      <div class={styles["container"]}>
        <article class={styles["sign-up-card"]}>
          <section>
            <h3>{t("signUp")}</h3>
            <label>{t("description")}</label>
          </section>
          <section>
            <button
              class={styles["sign-up-with-google"]}
              onClick={() => {
                core.userActions.signIn();
              }}
            >
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
