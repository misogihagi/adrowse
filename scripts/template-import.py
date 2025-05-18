import os
import shutil

locales_dir = "locales"
source_dir = "./source"

def main(dic):
    os.makedirs(locales_dir, exist_ok=True)

    for locale in dic:
        locale_path = os.path.join(locales_dir, locale)
        os.makedirs(locale_path, exist_ok=True)

        for template_dir in dic[locale]:
            for file_name in dic[locale][template_dir]:
                source_file_path = os.path.join(
                    source_dir,
                    locale,
                    template_dir,
                    dic[locale][template_dir][file_name],
                    "index.md",
                )
                destination_file_path = os.path.join(locale_path, file_name + ".md")

                shutil.copy2(source_file_path, destination_file_path)


dic = {
    "cy": {
        "templedi": {
            "decision-record-template-by-edgex": "templed-cofnod-penderfyniad-gan-edgex",
            "decision-record-template-by-jeff-tyree-and-art-akerman": "templed-cofnod-penderfyniad-gan-jeff-tyree-and-art-akerman",
            "decision-record-template-by-michael-nygard": "templed-cofnod-penderfyniad-gan-michael-nygard",
            "decision-record-template-for-alexandrian-pattern": "templed-cofnod-penderfyniad-ar-gyfer-patrwm-alecsandraidd",
            "decision-record-template-for-business-case": "templed-cofnod-penderfyniad-ar-gyfer-achos-busnes",
            "decision-record-template-of-the-madr-project": "templed-cofnod-penderfyniad-y-prosiect-madr",
            "decision-record-template-using-planguage": "templed-cofnod-penderfyniad-gan-planguage",
        },
    },
    "en": {
        "templates": {
            "decision-record-template-by-edgex": "decision-record-template-by-edgex",
            "decision-record-template-by-jeff-tyree-and-art-akerman": "decision-record-template-by-jeff-tyree-and-art-akerman",
            "decision-record-template-by-michael-nygard": "decision-record-template-by-michael-nygard",
            "decision-record-template-for-alexandrian-pattern": "decision-record-template-for-alexandrian-pattern",
            "decision-record-template-for-business-case": "decision-record-template-for-business-case",
            "decision-record-template-of-the-madr-project": "decision-record-template-of-the-madr-project",
            "decision-record-template-using-planguage": "decision-record-template-using-planguage",
        }
    },
    "es": {
        "plantillas": {
            "decision-record-template-by-edgex": "plantilla-de-registro-de-decisión-de-edgex",
            "decision-record-template-by-jeff-tyree-and-art-akerman": "plantilla-de-registro-de-decisión-de-jeff-tyree-y-art-akerman",
            "decision-record-template-by-michael-nygard": "plantilla-de-registro-de-decisión-de-michael-nygard",
            "decision-record-template-for-alexandrian-pattern": "plantilla-de-registro-de-decisión-para-el-patrón-alejandrino",
            "decision-record-template-for-business-case": "plantilla-de-registro-de-decisión-para-caso-de-negocio",
            "decision-record-template-of-the-madr-project": "plantilla-de-registro-de-decisión-del-proyecto-madr",
            "decision-record-template-using-planguage": "plantilla-de-registro-de-decisión-usando-planguage",
        }
    },
    "fr": {
        "modèles": {
            "decision-record-template-by-edgex": "modèle-denregistrement-de-décision-par-edgex",
            "decision-record-template-by-jeff-tyree-and-art-akerman": "modèle-denregistrement-de-décision par jeff-tyree-et-art-akerman",
            "decision-record-template-by-michael-nygard": "modèle-denregistrement-de-décision-par michael-nygard",
            "decision-record-template-for-alexandrian-pattern": "modèle-denregistrement-de-décision-pour-le modèle-alexandrin",
            "decision-record-template-for-business-case": "modèle-denregistrement-de-décision pour-lanalyse-de-rentabilisation",
            "decision-record-template-of-the-madr-project": "modèle-denregistrement-de-décision-du-projet-madr",
            "decision-record-template-using-planguage": "modèle-denregistrement-de-décision utilisant-plangage",
        }
    },
    "ja": {
        "テンプレート": {
            "decision-record-template-by-edgex": "EdgeX Foundryによる意思決定記録テンプレート",
            "decision-record-template-by-jeff-tyree-and-art-akerman": "Jeff TyreeとArt Akermanによる意思決定記録テンプレート",
            "decision-record-template-by-michael-nygard": "Michael Nygard による意思決定記録テンプレート",
            "decision-record-template-for-alexandrian-pattern": "アレクサンドリアパターンの意思決定記録テンプレート",
            "decision-record-template-for-business-case": "ビジネスケース用意思決定記録テンプレート",
            "decision-record-template-of-the-madr-project": "madrプロジェクトの意思決定記録テンプレート",
            "decision-record-template-using-planguage": "Planguageを使用した意思決定記録テンプレート",
        }
    },
    "ko": {
        "템플릿": {
            "decision-record-template-by-edgex": "결정 기록 템플릿별 edgex",
            "decision-record-template-by-jeff-tyree-and-art-akerman": "의사결정 기록 템플릿-by-jeff-tyree-and-art-akerman",
            "decision-record-template-by-michael-nygard": "의사 결정 기록 템플릿-by-michael-nygard",
            "decision-record-template-for-alexandrian-pattern": "알렉산드리아 패턴에 대한 결정 기록 템플릿",
            "decision-record-template-for-business-case": "비즈니스 사례에 대한 의사결정 기록 템플릿",
            "decision-record-template-of-the-madr-project": "madr 프로젝트의 결정 기록 템플릿",
            "decision-record-template-using-planguage": "의사결정 기록 템플릿 사용 p언어",
        }
    },
}

main(dic)
