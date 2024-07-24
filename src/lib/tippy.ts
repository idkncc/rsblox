import runTippy from 'tippy.js';
import type { Props } from 'tippy.js';

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
            interactive: true,
            duration: [50, 200],
            animateFill: true,
            animation: "shift-away-subtle"
        }, props)
    );

    return {
        destroy,
        update
    };
};
