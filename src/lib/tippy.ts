import runTippy from 'tippy.js';
import type { Props } from 'tippy.js';

/**
 * An interface of [Tippy.js Props](https://atomiks.github.io/tippyjs/v6/html-content/)
 */
export type TippyProps = Partial<Omit<Props, 'trigger'>>;

export interface TippyReturn {
    update: (newProps: TippyProps) => void;
    destroy: () => void;
}
export type Tippy = (element: HTMLElement, props?: TippyProps) => TippyReturn;

export const tippy: Tippy = (element, props) => {
    const { destroy, setProps: update } = runTippy(
        element,
        Object.assign({
            placement: "bottom-end",
            interactive: true,
            duration: [0, 100],
            animateFill: true,
        }, props)
    );

    return {
        destroy,
        update
    };
};
