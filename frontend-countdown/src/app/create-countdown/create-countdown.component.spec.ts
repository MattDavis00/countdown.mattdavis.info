import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { CreateCountdownComponent } from './create-countdown.component';

describe('CreateCountdownComponent', () => {
  let component: CreateCountdownComponent;
  let fixture: ComponentFixture<CreateCountdownComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ CreateCountdownComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(CreateCountdownComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
